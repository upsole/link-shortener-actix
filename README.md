# Exploring Actix-Web and Diesel

Quick hack of an URL shortener to practice both crates.

* POST on /shorten expects payload of JSON `{"url": String}`
* GET on /redirect/{short_url} redirects to the original URL 

Database is dockerized, 'launch_docker_db.sh' starts it through docker-compose and
runs diesel migrations. You need to set up DATABASE_URL in `.env`. 


Validation/Input sanitizing needs to be improved and so does error handling.
Could also use some logic to handle collision of generated URLS
