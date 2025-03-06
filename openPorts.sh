#!/usr/bin/env bash

sudo iptables -A INPUT -p tcp --dport 8081 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 8554 -j ACCEPT
