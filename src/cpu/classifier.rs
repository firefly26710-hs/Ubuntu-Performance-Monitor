const THREAD_USAGE:usize = 20;//待定
const THREAD_NUMBER:usize = 12;
struct Information{
    name:String,
    threads_usage:[String; THREAD_NUMBER*THREAD_USAGE]
}