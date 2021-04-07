# actix-covid19

Just a sample project to learn Rust lang.

# Start http server
```
cargo run
```


# Unit test & Integration test 
```
cargo test
```

# Public path

`GET` `/health` is public path and will return Ok


# Protected path 
```
will need basic authentication header with default (username: user, password: user123)
to access data
```

`GET` `/app/covid19` get covid19 cases statistic and then group the data by province

`GET` `/app/hash/{value_to_be_hashed}` will return hashed value from url param

`PUSH` `/app/user` to add new user

Payload
```
{
    "username":"username_to_be_added",
    "password":"password"
}
```
