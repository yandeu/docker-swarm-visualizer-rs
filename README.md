# Docker Swarm Visualizer (rust)

<a href="https://raw.githubusercontent.com/yandeu/docker-swarm-visualizer-rs/main/readme/screenshot.png">
   <img width="850" alt="screenshot" src="https://raw.githubusercontent.com/yandeu/docker-swarm-visualizer-rs/main/readme/screenshot.png">
</a>

## About

This is the same as [Docker Swarm Visualizer](https://hub.docker.com/r/yandeu/visualizer), but written in Rust and view-only.  
Because it is written in Rust, a running container only uses about 2MB memory, compared to >40MB in Nodejs.

## Links

- [`github.com`](https://github.com/yandeu/docker-swarm-visualizer-rs)
- [`hub.docker.com`](https://hub.docker.com/r/yandeu/visualizer-rs)

## Getting Started

1. Make sure you are using docker in swarm mode (`docker swarm init`).

```markdown
# make sure the required ports are open

TCP port 2377 for cluster management communications  
TCP and UDP port 7946 for communication among nodes  
UDP port 4789 for overlay network traffic
```

2. Make sure you can access your swarm on port **9510/tcp**.

3. Make sure the nodes can communicate with each other on port **9511/tcp**.

4. Deploy the Visualizer

   ```bash
   # Download the Stack File (from GitHub)
   curl https://raw.githubusercontent.com/yandeu/docker-swarm-visualizer-rs/main/visualizer-rs.stack.yml -o visualizer-rs.stack.yml

   # Deploy the Stack
   docker stack deploy -c visualizer-rs.stack.yml visualizer-rs
   ```

5. Open the Visualizer Dashboard  
   [`http://127.0.0.1:9510`](http://127.0.0.1:9510) or [`http://[NODE_IP]:9510`](http://[NODE_IP]:9510)
