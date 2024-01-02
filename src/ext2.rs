mod block;
struct EXT2FS{
    boot_block : Boot_Block,
    block : vec![Block_Group],
    user_name : String,
    password : String
}

