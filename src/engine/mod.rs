
use std::collections::hash_map::HashMap;

struct ValueInfo{
    Length :u64, 
    Start : u64,
}
struct Engine{
    KeyMap : HashMap<String,ValueInfo>,
    
}