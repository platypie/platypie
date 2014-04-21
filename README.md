# What is <a href="http://platypie.org">Platypie</a>?
Platypie is an [open source](https://github.com/platypie "our git"), and truly crossplatform mesh networking initiative inspired by **[the world's favourite monotreme](http://en.wikipedia.org/wiki/Platypus)**, the Platypus!  
  
Just like our semi-aquatic mascot, mesh networking software needs to evolve to be able to go *anywhere*. It must be compact enough to run on mobile devices, yet powerful enough to scale up in more 
demanding circumstances. Furthermore, it must be able to withstand determined aggressors. You probably think the platypus is adorable, and so do we. Many people do not realize that this wee mammal
also packs a venomous stinger which [can leave would be attackers in agony for months on end](http://en.wikipedia.org/wiki/Platypus_venom "platypus venom").  
<br>
We believe not only in tough cryptographic standards, but in providing a network architecture that cannot be compromised.  
  
Platypie is being developed in [Rust](http://www.rust-lang.org/ "Rust Lang"), a modern, multi-paradigm programming language that we think is perfect for solving the issues of the modern web. Being web developers, we are also fond of [Javascript](http://nodejs.org/ "nodejs"). If you're interested in contributing, learning either would be a good place to start. Fixing the web is not just a technical problem, it is also very much a social problem. If you are skilled in graphic design, communication, or network hardware configuration, by all means, [get involved](https://github.com/platypie/platypie#get-involved)!  
  
The value of networking software is a product of its adoption. Although it's critical that a project like this produce working code, we are committed to first creating a viable set of specifications which will allow others to create alternatives which are interoperable with other nodes in the network. Skeptical? Western naturalists thought the platypus was a hoax when they first heard of it too!  
  
## Install it  
  
Most of these commands will work on any Unix-based system. Some of them assume a Debian base. This is alpha software, and it doesn't do much at the moment. It's only worth the installation if you intend to help us test it. While we're assembling the core, things are guaranteed to change around.  
If you'd like support on your OS or architecture, your best option is to get involved with testing.  
So far it has been installed on:  
* Mac OSX
* Debian 7
* Ubuntu 12.04 && 14.04
* Mint
* Arch (but not on arm)
* Gentoo
* Windows support is being investigated

Start by installing whichever stable binary of [Rust](http://www.rust-lang.org/install.html) is available for your system.  
  
Satisfy the dependencies.  
`sudo apt-get install git build-essential make`  
  
Install libsodium:  
if your OS doesn't offer an official package.  

```bash
wget https://download.libsodium.org/libsodium/releases/libsodium-0.4.5.tar.gz
tar xvzf libsodium-0.4.5.tar.gz
cd libsodium-0.4.5
sudo su
./configure --prefix=/usr && make && make check && make install
exit
```
  
Change directories to where you'd like to install platypie, then:  
```bash
git clone https://github.com/platypie/platypie.git  
cd platypie
git submodule init
git submodule update # get sodiumOxide
cd libs/sodiumoxide
rustc -O src/sodiumoxide/lib.rs
cd ../..
ldconfig # make sure the new libs are linked
make exe
```
  
## Run it  
`make run`  
  
## Get involved  
Join us on [Freenode](http://webchat.freenode.net/?channels=%23platypie "freenode's webchat") in #platypie  
  
If you can program, and intend to submit code to be included in the official repository, start by first reading HACKING.md on our coding practices.  
  
If you don't program, or if you think this is over your head, we still need:  
* artwork
* documentation
* testing on various architectures/systems
* third party applications
* input (perhaps most importantly)
