<br />
<h1 align="center">Fullstack Rust GPT-3 Project</h1>

## About The Project

This project is an educational fullstack application written in Rust, using the Yew.rs framework for the frontend and the Rocket framework for the backend. It is designed to allow users to make requests to OpenAI, and have the responses returned to them. The requests are protected by unique keys, which must be added by an admin, and each key has 10 questions associated with it. On the frontend, the user must provide the key and the question to the AI in order for the request to be processed. The entire project was created for educational purposes, and a series of YouTube videos have been recorded to demonstrate the project and its features.

### Built With

- [Rust](https://www.rust-lang.org/)
- [Rocket](https://rocket.rs/)
- [Yew](https://yew.rs/)
- [SQLite](https://sqlite.org/index.html)
- [OpenAI](https://openai.com/)

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [SQLite3](https://sqlite.org/download.html)
- [Trunk](https://yew.rs/docs/getting-started/introduction)

### Installation

1. Open backend directory

2. Create a new Database instance with the following commands

```
sqlite3 ./keys.db
create table keys (id varchar(255), left int);
.q
   
```

3. Create a `.env` file in the root folder and add the
   following variables:

```
   OPENAI_KEY=<OPENAI_API_KEY>
   ADMIN_KEY=<RANDOM_ADMIN_PASSWORD>
```

Hint: You can get your own OpenAI API key from https://beta.openai.com/

4. Inside of the backend directory run command `cargo run`

5. Open second instance of terminal and go to frontend directory

6. Run `trunk serve` command

### Adding Question Keys

In order to add new keys for users to use call the following *POST* request

```
URL: http://127.0.0.1:8000/api/v1/add_key
BODY: {
    "password": <ADMIN_KEY>
    }
```

### Querying all available keys

In order to see all keys call the following *GET* request

```
URL: http://127.0.0.1:8000/api/v1/query_all
BODY: {
    "password": <ADMIN_KEY>
    }
```

### Videos

1. (https://www.youtube.com/watch?v=DBFZgWI9vME)[https://www.youtube.com/watch?v=DBFZgWI9vME]
2. (https://www.youtube.com/watch?v=LYNLb_YDDzE)[https://www.youtube.com/watch?v=LYNLb_YDDzE]
3. (https://www.youtube.com/watch?v=r8s0nbCKQ_g)[https://www.youtube.com/watch?v=r8s0nbCKQ_g]