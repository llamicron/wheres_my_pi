# Where's my Pi?
This is a small application that sends a machines IP addresses to your discord webhook when it boots up. I use it to keep track on my raspberry Pis. They get new IP addresses every once in a while, and I hate having to go to my modem homepage or connect a keyboard and monitor just to figure out the IP so I can remote into them anyway. Now you can just restart them and it'll send you the IP.

# Usage
Install with cargo

```
$ cargo install wheres_my_pi
```

Set up a discord webhook and put it in `~/.wheres_my_pi`. To set up a webhook, go to the discord server you're using and click the little cog next to the channel you want things sent to. There should be an "integrations" tab. Follow the instructions from there.

I would recommend making a channel just for this so you don't get spammed.

After the webhook is in `~/.wheres_my_pi`, run it manually like this:
```s
$ wheres_my_pi
```

## Set Autostart
Put the command to run this program in `/etc/rc.local` to have it run on system startup.

