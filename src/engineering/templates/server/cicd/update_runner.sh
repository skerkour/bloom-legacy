docker pull gitlab/gitlab-runner:latest
docker stop gitlab-runner && docker rm gitlab-runner
docker run -d --name gitlab-runner --restart always \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v `pwd`/runner:/etc/gitlab-runner \
  gitlab/gitlab-runner:latest
