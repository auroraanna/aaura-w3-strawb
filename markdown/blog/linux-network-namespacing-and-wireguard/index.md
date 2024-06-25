```toml
title = "🐧 Linux network namespacing and WireGuard: tunneling only some apps though a VPN"
date_published = "2024-06-25T16:11:08+02:00"
description = "Anna describes the experience and struggles trying to set up only some apps to run all their traffic through a VPN on Linux."
keywords = ["guide", "linux", "torrenting", "vpn", "network-namespacing", "wireguard", "nixos", "tunnel"]

[atom_id_parts]
email = "anna@annaaurora.eu"
object = "0"
```
Anna tried to find a way how to run only some applications' traffic though a VPN (one of those from a commercial provider), like a torrent client or a web browser. This article explains how to do it on Linux. While there are also NixOS configuration snippets provided, you can apply this knowledge to others distros as well. Anna has no idea if this sort of thing is possible on other operating systems like Windows or macOS.

While the qbittorrent (torrent) client supports forcing all its network traffic to run through a specific network interface (Preferences > Advanced > Network interface), like the one of a VPN, Anna didn't find out how to configure her networking so it wouldn't run all traffic over that network interface. I.e. wg-quick config files support an `AllowedIPs` attribute under `[Peer]` and setting it to `0.0.0.0/0` routes all traffic through that interface but setting it to the ip address (with subnetmask) of the network interface, e.g. `10.2.0.2/32`, breaks it. This is part of the reason Anna opted to go with network namespacing. Networking namespacing also allows you force abitrary applications to run through your VPN, they don't have to support setting a network interface like qbittorrent.

# How to do it

Our network namespace is called `tun0` and our network interface is also called `tun0`. The ip address of the network namespace is `10.2.0.2` (IPv4 addresses used for point to point connections typically start with `10`.). You're going to need the `ip` executable and the `wg` executable. Generally, `ip -n [name of network namespace] [arguments to ip]` is used to run `ip` commands inside of the network namespace and `ip netns exec [name of network namespace] [command]` is used to run other commands inside of the network namespace though the latter can also be used to run `ip` commands inside of the network namespace.

Create the network namespace:
```sh
ip netns add tun0
```

Create the wireguard network interface:
```sh
ip link add tun0 type wireguard
```

Move the wireguard network interface into the network namespace:
```sh
ip link set tun0 netns tun0
```

Add the ip address to the network interface (now inside the network namespace):
```sh
ip -n tun0 address add 10.2.0.2/32 dev tun0
```

Configure the network interface with wireguard:
```sh
ip netns exec tun0 wg setconf tun0 [path to your wireguard config file]
```
The wireguard config (should end with `.conf`, can usually be obtained at VPN providers) though be sure to comment out or remove the attributes `Address` and `DNS` from the `[Interface]` section as those are only supported with wg-quick, and we can't use wg-quick because it doesn't suport network namespacing. If you tried to set up a wireguard interface with wg-quick, it would fully configure it so that traffic could already run over it before you can even move it into the network namespace which is especially problematic because it configures all traffic to be routed through the wireguard interface if you set `AllowedIPs` to `0.0.0.0/0`.

Set the default route to be through the wireguard network interface:
```sh
ip -n tun0 route add default dev tun0
```

Set the network interface to up:
```sh
ip -n tun0 link set tun0 up
```

If you're using systemd-resolved for DNS like me then you'll have the problem that applications inside the network namespace can't contact the systemd-resolved DNS server at because it is outside the network namespace. If your VPN provider has a DNS server at the other end of the tunnel, e.g. `10.2.0.1` then you can create a file at `/etc/netns/tun0/resolv.conf` with the contens `nameserver 10.2.0.1`. This will override your systemd config's `/etc/resolv.conf` which points to the systemd-resolved DNS server only for the network namespace.

If you need localhost inside your network namespace, just up the loopback interface and it will automatically assign addresses:
```sh
ip -n tun0 link set lo up
```

If you don't want to have the namespace and the network interfaces inside it anymore:
```sh
ip -n tun0 route del default dev tun0
ip -n tun0 link del tun0
ip netns del tun0
```

If you want to automate this, you could set up a systemd service. Here's how to do it with NixOS (pkgs being nixpkgs):
```nix
{ pkgs, ... }

{
  systemd.services.netns-tun0 = {
    description = "Network namespace that can only access the internet through wireguard";
    requires = [ "network-online.target" ];
    path = with pkgs; [ iproute wireguard-tools ];
    script = ''
      set -x
      ip netns add tun0
      ip link add tun0 type wireguard
      ip link set tun0 netns tun0
      ip -n tun0 address add 10.2.0.2/32 dev tun0
      ip netns exec tun0 \
        wg setconf tun0 [path to your wireguard config file]
      ip -n tun0 link set tun0 up
      ip -n tun0 route add default dev tun0

      ip -n tun0 link set lo up
    '';
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
      ExecStop = pkgs.writeShellScript "wg-down" ''
        ip -n tun0 route del default dev tun0
        ip -n tun0 link del tun0
        ip netns del tun0
      '';
    };
  };

  environment.etc."netns/tun0/resolv.conf" = {
    text = "nameserver 10.2.0.1";
    mode = "0444";
  };
}
```
The existing NixOS systemd service wireguard and wg-quick can't be directly used for this: The wireguard service can only movs it's network interface into the network namespace but can't create the network namespace. I didn't find any other NixOS options for creating network namespaces either. The wireguard service doesn't support the `DNS` config attribute for `[Interface]` either like wg-quick but the wg-quick service and executable don't have network namespacing support though there is [a fork that does](https://github.com/dadevel/wg-netns) but it's not in nixpkgs. Creating a new systemd service for this helps me better understand what wg-quick abstracts.

Now you can just run any application, inside the network namespace:
```
ip netns exec tun0 nslookup annaaurora.eu
```

Although if you are in trying to run a graphical application and you're not logged in as root or if you don't want to run as it as root then use `su`:
```
sudo ip netns exec tun0 su [your username] -c 'qbittorrent'
```
