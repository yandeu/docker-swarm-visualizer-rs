// https://github.com/yandeu/docker-swarm-visualizer/blob/main/src/lib/dns.ts
// export const agentDNSLookup = (): Promise<DNS[]> => {
//   // We send ['127.0.0.1'] when we (most probably) are developing the app locally.

//   return new Promise(resolve => {
//     dns.lookup('tasks.agent', { all: true }, (err, addresses, family) => {
//       if (addresses) {
//         const addr = addresses.map(a => a.address)
//         return resolve(addr.length > 0 ? addr : ['127.0.0.1'])
//       }
//       return resolve(['127.0.0.1'])
//     })
//   })
// }

use dns_lookup::{lookup_addr, lookup_host};

pub fn agent_dns_lookup() -> Vec<String> {
    let hostname = "tasks.agent";
    let ips: Vec<std::net::IpAddr> = match lookup_host(hostname) {
        Ok(ips) => ips,
        Err(_e) => return vec![String::from("127.0.0.1")],
    };

    if ips.is_empty() {
        return vec![String::from("127.0.0.1")];
    }

    let ips_string: Vec<String> = ips.iter().map(|ip| ip.to_string()).collect();
    ips_string
}
