dfx identity use default 
dfx canister call stableImp_backend create_post  '(record {title = "hello" ; data = "test1" ; created_by = "user"})'
dfx canister call stableImp_backend get_post 
dfx canister call stableImp_backend remove_post

dfx identity new minter
dfx identity use minter
dfx canister call stableImp_backend create_post  '(record {title = "hello" ; data = "test1" ; created_by = "user"})'
dfx canister call stableImp_backend get_post 
dfx canister call stableImp_backend remove_post
