cargo install diesel_cli --no-default-features --features postgres


diesel setup


diesel migration generate videos
diesel migration generate users
diesel migration generate views


diesel migration run
