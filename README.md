imageflip.com parser crate 🦀



* simple Example
```rust
use imgflipparser::start;

fn main() {
    for page_id in 1..10 {
        for i in start(Some(page_id as i32)) {
            for (key, value) in i {
                println!("{}:{}", key, value);
            }
        }
    }
}
```


Enable the web API
```rust
Cargo.toml
---
imgflipparser = { version = "0.1.3", features = ["webapi"] }
```

```rust
use imgflipparser::start_api;

fn main() {
    start_api();
}

```

endpoint: `GET /{ID}` - page id

```bash
$ curl http://localhost:8080/30 | jq
[{"url":"//i.imgflip.com/62gzj8.jpg","alt":"*Creative Title 3* |  General: Shoot now; The guy named Now: | image tagged in memes,surprised pikachu,funny,general,shoot | made w/ Imgflip meme maker","id":"30"},{"url":"//i.imgflip.com/62gzj8.jpg","alt":"*Creative Title 3* |  General: Shoot now; The guy named Now: | image tagged in memes,surprised pikachu,funny,general,shoot | made w/ Imgflip meme maker","id":"30"},{"id":"30","url":"//i.imgflip.com/634ocp.jpg","alt":"that uncle's a gamer |  THAT ONE UNCLE TRYING TO ENTERTAIN THE KIDS | image tagged in i sold my son on ebay | made w/ Imgflip meme maker"},{"url":"//i.imgflip.com/634ocp.jpg","alt":"that uncle's a gamer |  THAT ONE UNCLE TRYING TO ENTERTAIN THE KIDS | image tagged in i sold my son on ebay | made w/ Imgflip meme maker","id":"30"},{"url":"//i.imgflip.com/632zyn.jpg","alt":"Waiting Skeleton |  ME WAITING FOR MY FRIEND TO REPLY TO MY MESSAGES | image tagged in memes,waiting skeleton | made w/ Imgflip meme maker","id":"30"},{"url":"//i.imgflip.com/632zyn.jpg","id":"30","alt":"Waiting Skeleton |  ME WAITING FOR MY FRIEND TO REPLY TO MY MESSAGES | image tagged in memes,waiting skeleton | made w/ Imgflip meme maker"},{"id":"30","url":"//i.imgflip.com/634az4.jpg","alt":"He Ain't No Comedian |  CHUCK NORRIS CAN'T TELL JOKES; CAUSE NO ONE WOULD SURVIVE THE PUNCHLINE | image tagged in memes,chuck norris | made w/ Imgflip meme maker"},{"id":"30","alt":"He Ain't No Comedian |  CHUCK NORRIS CAN'T TELL JOKES; CAUSE NO ONE WOULD SURVIVE THE PUNCHLINE | image tagged in memes,chuck norris | made w/ Imgflip meme maker","url":"//i.imgflip.com/634az4.jpg"},{"alt":"Literally true |  Clash Of Clans/ Clash Royale startup sound | image tagged in the loudest sounds on earth | made w/ Imgflip meme maker","url":"//i.imgflip.com/6333yb.jpg","id":"30"},{"alt":"Literally true |  Clash Of Clans/ Clash Royale startup sound | image tagged in the loudest sounds on earth | made w/ Imgflip meme maker","url":"//i.imgflip.com/6333yb.jpg","id":"30"}]
```


### License
GPLv2
