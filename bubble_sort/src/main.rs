fn main() {
    let mut arr:[u32 ; 5]=[5,7,6,2,3];
    let len=arr.len();
    for _i in 0..len{
        for j in 0..(len-1){
            if arr[j]>arr[j+1]{
                let temp:u32=arr[j];
                arr[j]=arr[j+1];
                arr[j+1]=temp
            }
        }
    }
    println!("{:?}",arr)

}
