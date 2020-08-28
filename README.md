## Live on Heroku
[See it here.](https://triangle-sss-solver.herokuapp.com)

## Use Docker to build and run

Clone the repo and set it as your working directory.

```
$ docker build -t ANY_IMAGE_NAME:v1
$ docker run -p 8080:8080 --rm -it -v $(pwd):/app ANY_IMAGE_NAME:v1
```
The server should be running at http://localhost:8080
