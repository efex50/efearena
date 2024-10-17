

const ARENA_SIZE:usize = 1024;



type ArenaSector = Box<[u8;ARENA_SIZE]>;

// todo 
// get ptr as *mut Option<Box<ArenaSector>>
// then return 0 if option is none

#[derive(Debug)]
pub struct WriteSector{
    sector:*mut ArenaSector,
    data:Vec<u8>,
    start:usize
}
impl WriteSector {
    pub fn write(&self){
        let mut ctr = self.start;

        let sector = unsafe {
            self.sector.as_mut().unwrap()
        };
        for x in &self.data{
            sector[ctr] = *x;
            ctr +=1;
        }

    }
}
// todo 
// get ptr as *mut Option<Box<ArenaSector>>
// then return 0 if option is none
#[derive(Debug)]
pub struct ReadSector{
    sector:*mut ArenaSector,
    start:usize,
    len:usize,
}
impl ReadSector {
    pub fn read(&self) -> Vec<u8>{

        let mut v = Vec::new();
        let sector: &mut Box<[u8; 1024]> = unsafe {self.sector.as_mut().unwrap()};
        for x in self.start..self.len{
            v.push(sector[x]);
        }
        v

    }
}


#[derive(Debug)]
pub struct Arena{
    arena_inner:Vec<Option<ArenaSector>>
}
impl Arena {
    pub fn new() -> Self{
        let v: Vec<Option<ArenaSector>> = Vec::new();
        Self {
            arena_inner:v
        }
    }
    
    fn fill_sectors_untill(&mut self,end:usize){
        for x in self.arena_inner.len()..end+1{
            if x == end{
                self.arena_inner.push(Some(Box::new([0;ARENA_SIZE])));
                break;
            }
            match self.arena_inner.get(x) {
                Some(_) => (),
                None => {
                    self.arena_inner.push(None);
                },
            }
        }
    } 
    pub fn write(&mut self,start:usize,arr:Vec<u8>){

        let (start_sector,mut ptr,end_sector) = {
            let arena_sector = if start == 0{
                0
            }
            else {
                start / ARENA_SIZE
            };

            let end_sector = if start+arr.len() == 0{
                0
            }
            else {
                (start + arr.len()) / ARENA_SIZE
            };

            let ptr = if start == 0 {
                0
            }
            else{
                start % ARENA_SIZE 
            };
            (arena_sector,ptr,end_sector)
        };

        let a = self.arena_inner.get(end_sector);
        if a.is_none(){
            self.fill_sectors_untill(end_sector);
        }
        // set data structures like vec<ArenaSector>
        if end_sector == start_sector{
            if self.arena_inner.get(start_sector).unwrap().is_none(){
                self.arena_inner[start_sector] = Some(Box::new([0_u8;ARENA_SIZE])); 
            }
            let a = self.arena_inner.get(start_sector).unwrap();
            let b = a.as_ref().unwrap();
            let sector_ptr =  b as *const ArenaSector as usize as *mut ArenaSector;

            WriteSector { sector: sector_ptr, data: arr, start: ptr }.write();
            return;

        }else {
            let mut s1 = ARENA_SIZE - ptr;
            let mut v = Vec::new();
            let mut current_sector = start_sector;
            for x in arr{

                if s1 == 0{
                    let sector_ptr = {
                        if self.arena_inner.get(current_sector).unwrap().is_none(){
                            self.arena_inner[current_sector] = Some(Box::new([0_u8;ARENA_SIZE])); 
                        }
                        let a = self.arena_inner.get(current_sector).unwrap();
                        let b = a.as_ref().unwrap();
                        b as *const ArenaSector as usize as *mut ArenaSector
                    };
                    WriteSector { sector: sector_ptr, data: v.clone(), start: ptr }.write();
                    current_sector += 1;
                    v.clear();
                    v.push(x);
                    s1 = ARENA_SIZE;
                    ptr = 0;
                }else {
                    v.push(x);
                    s1 -= 1;
                }
            }
            if !v.is_empty(){
                let sector_ptr = {
                    if self.arena_inner.get(current_sector).unwrap().is_none(){
                        self.arena_inner[current_sector] = Some(Box::new([0_u8;ARENA_SIZE])); 
                    }
                    let a = self.arena_inner.get(current_sector).unwrap();
                    let b = a.as_ref().unwrap();
                    b as *const ArenaSector as usize as *mut ArenaSector
                };

                WriteSector{
                    data:v,
                    sector:sector_ptr,
                    start:0
                }.write();
            }
        }
    }
    pub fn read(&self,start:usize,len:usize) -> Vec<u8>{
        let len = len;
        let (start_sector,mut ptr,end_sector) = {
            let arena_sector = if start == 0{
                0
            }
            else {
                start / ARENA_SIZE
            };

            let end_sector = if start+len == 0{
                0
            }
            else {
                (start + len) / ARENA_SIZE
            };

            let ptr = if start == 0 {
                0
            }
            else{
                start % ARENA_SIZE 
            };
            (arena_sector,ptr,end_sector)
        };


        let mut readed: Vec<Vec<u8>> = Vec::new();

        if end_sector == start_sector{
            if self.arena_inner.get(start_sector).is_none()  {
                let r = vec![0_u8;len];
                readed.push(r);
                
            } else if self.arena_inner.get(start_sector).unwrap().is_none(){
                let r = vec![0_u8;len];
                readed.push(r);

            } else{
                let a = self.arena_inner.get(start_sector).unwrap();
                let b = a.as_ref().unwrap();
                let sector_ptr =  b as *const ArenaSector as usize as *mut ArenaSector;

                let r = ReadSector { sector: sector_ptr, start: ptr,len:ptr+len }.read();
                readed.push(r);
            }

        }else {
            let mut local_len: usize;
            let end_len = (start+len) % ARENA_SIZE;
            'l:for x in start_sector..end_sector+1{
                
                let sector_ptr = {
                    if self.arena_inner.get(start_sector).is_none()  {
                        let r = vec![0_u8;len];
                        readed.push(r);
                        break 'l;                        
                    } else if self.arena_inner.get(start_sector).unwrap().is_none(){
                        let r = vec![0_u8;len];
                        readed.push(r);
                        break 'l;
                    }
                    let a = self.arena_inner.get(x).unwrap();
                    let b = a.as_ref().unwrap();
                    b as *const ArenaSector as usize as *mut ArenaSector
                };

                if x == end_sector{
                    local_len = if len == 0 {
                        0
                    }
                    else{
                        end_len 
                    };
                }else {
                    local_len = ARENA_SIZE;
                }

                
                let r = ReadSector{
                    start:ptr,
                    sector:sector_ptr,
                    len:local_len
                };
                let r = r.read();
                readed.push(r);

                ptr = 0;


            }
        }
        let a:Vec<u8> = readed.concat();

        a
    }
}


mod tests{
    #[allow(unused)]
    use crate::{Arena, ARENA_SIZE};


    #[test]
    fn arena_write_test(){
        let mut a = Arena::new();
        let v = [1_u8;ARENA_SIZE * 4].to_vec();
        a.write(4096, v);
        println!("{:?}",a)
         
    }
    #[test]
    fn arena_write_test2(){
        let mut a = Arena::new();
        let v = [1_u8;ARENA_SIZE / 2].to_vec();
        a.write(20, v);
         
    }
    
    
    #[test]
    fn arena_read_test(){
        let mut a = Arena::new();
        let v = [1_u8;ARENA_SIZE * 4].to_vec();
        a.write(4096, v);
        let r = a.read(4096, ARENA_SIZE * 4);
        println!("len:{:?}",r.len());
        println!("arena : {:?}",r);
    }
    #[test]
    fn arena_read_test2(){
        let a = Arena::new();
        // let v = [1_u8;ARENA_SIZE].to_vec();
        // a.write(0, v);666
        let r = a.read(4096, ARENA_SIZE * 4);
        println!("len:{:?}",r.len());
        println!("arena  : {:?}",a);
        println!("readed : {:?}",r);
    }
    #[test]
    fn arena_len_test(){
        let c = [0_u8;ARENA_SIZE];
        println!("{:?}",c[1023])
    }    
    

    #[test]
    fn malloc_test(){
        let mut a = Box::new([2_u8;ARENA_SIZE]);
        let b = &mut a as *mut Box<[u8; 1024]> as usize as *mut &mut [u8; 1024];
        unsafe {
            let c = b.as_mut().unwrap();
            for x in 0..101_u8{
                c[x as usize] = x;
            }
        }
        println!("{:?}",a);
    }
}