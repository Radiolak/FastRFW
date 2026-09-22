# FastRFW
This project is a basic and educational project, designed to help me better understand how system calls work for networking purposes. This project uses netfilter hooks to manipulate packets. Only for IPv4.

# The basic usage of the firewall:
	- Blocking or allowing incoming network traffic by PORT, IP, or protocol filters
	- Blocking or allowing outgoing network traffic by PORT, IP, or protocol filters
	- Sorting rule priority (weight) relative to one another
	- Compatibility with 1 WAN interface and multiple LAN interfaces

# Requirements:
	- Debian-based Linux distribution
	- Kernel version: 6.0 or higher

# Usage:
  - Fastrfw -d [IN/OUT] -a [ADD/REMOVE] -p [protocol] -c [priority] -t [IP / PORT]

 
# Possible commands:

-h: Help & more information
-i: get filters

# Architecture cible:

# Structure of the project:

fastrfw/
|- cargo.toml
|- src/
|   |-- main.rs       # Syntax and argument validation
|   |-- lib.rs           # Calcul of the weight using priority.rs and add the rule to the table
|   |-- syscall.rs    # do the syscall and update the netfilter hook in the kernel
|   |--rules.rs/
|   |   |-- mod.rs   # interfaces and rules gestion
|   |   |-- priority.rs  


# How this will work:
  1: The kernel receives an IPv4 packet on the WAN interface or one of the LAN interfaces
  2: A Netfilter hook intercepts the packet and puts it into a queue
  3: fastrfw reads the packet using the recv() syscall
  4: The rules.rs module reads rules by priority
  5: Send the response (NF_ACCEPT or NF_DROP) to the kernel using the send() syscall

# Dependancies: 

	- CLAP: for cli parsing ----> take a look to native parsing solution: https://rust-cli.github.io/book/tutorial/cli-args.html
	- Libc or syscall for socket Netlink manipulaitons

# Description of the choice of the syscall queue:

The packet enters the kernel space, where a Netfilter hook intercepts it according to the firewall configuration. Netfilter blocks the packet and places it into a specific user-space queue (NFQUEUE). At this precise moment, the packet's traversal is entirely halted. fastrfw reads the packet payload and metadata from this queue via Netlink sockets. The packet remains frozen in the kernel memory until the Rust program processes the data and sends back an explicit verdict (NF_ACCEPT or NF_DROP).

# Performance:

This approach introduces a context-switching overhead between the kernel space and the user space for every single packet. Consequently, it is less performant than eBPF-based solutions (like Aya), which execute bytecode directly inside the kernel space and bypass traditional user-space queues entirely. However, for an educational and lightweight firewall, this performance trade-off is perfectly acceptable and significantly simplifies development.

# For security:

There is no security risk of packet leakage when using Netfilter Queue. The Linux kernel guarantees that no packet assigned to a queue can bypass the firewall or proceed through the network stack without a formal verdict from the user-space application. If fastrfw crashes or stops responding, the packets will simply pile up in the queue and be dropped or blocked (depending on the fail-safe configuration), ensuring that no unauthorized traffic ever passes through.

# What we need to implement to have a correct closing behavior for the application:

If the process stops, we need to integrate logic to intercept stop signals like SIGINT or SIGTERM; the program needs to cleanly close nfqueue properly.
In the systemd service, we need to implement a fail-safe script with options to block all traffic or restore the initial config. To do all of that, we need two modes. The user takes this decision during the installation of the software via the configuration file or configuration context on the 
first launch of the application, which can be modified afterward (maybe with a command argument):

• Fail-Closed: the traffic is blocked even if the firewall is not active
• Fail-Open: let all traffic pass

# Update on future releases:

NFQUEUE reviews each packet one by one synchronously, but if there is one packet that takes too long to be analyzed, the next one will wait in the queue. If the queue grows to its maximum size, the kernel will just reject new packets.
In future releases, it would be great to implement multi-threading logic.

# Sources: 

Linux kernel syscall for network:
https://linux-kernel-labs.github.io/refs/pull/189/merge/labs/networking.html

Versions of the linux kernel:
https://www.kernel.org/releases.html

Other documentation for networking on linux kernel: 
https://kernelnewbies.org/Linux_6.0#Networking

Rust syscall:
https://docs.rs/syscalls/latest/syscalls/

Rust syscall for x86_64 cpu:
https://docs.rs/syscalls/latest/syscalls/x86_64/enum.Sysno.html

Another way to do that without using kernel hooks: 
https://github.com/aya-rs/aya
