各个模块
-[ ] api doc
-[ ] auth
-[ ] data model
-[x] persist
-[x] migration
-[ ] log
-[ ] test
-[x] profiles

写法
- dto builder
- service fairing, replace request guard use db connector


api GET api.nodeplus.io/stats
{

code: 0,
message: "success",
data:{
total_assets: 12345,
total_rewards:12345,
total_running_time:12345,
total_node_count:12345
}
}




POST api.nodeplus.io/contact_us
request:
{
name: "my name",
email:"my email address",
message: "my message"
}
response:
{
code: 0,
message: "success",
}