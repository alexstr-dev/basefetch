# basefetch
A stupid fetch utility for linux that shouldnt be taken seriously even in the slightest.

# Note
 - (must have cpuid installed)

# How to build
1) git clone https://github.com/alexstr-dev/basefetch
2) cd basefetch
3) cargo build --release
4) target/release/basefetch

# How to use for the technically illiterate people
1) cd into .config
2) mkdir basefetch
3) cd basefetch
4) nvim config (DONT add file extensions if you actually want it to work)
5) paste in the example text below and write your config

<pre>
  ____                   _____    _       _       ____    ___
| __ )  __ _ ___  ___  |  ___|__| |_ ___| |__   |___ \  / _ \
|  _ \ / _` / __|/ _ \ | |_ / _ \ __/ __| '_ \    __) || | | |
| |_) | (_| \__ \  __/ |  _|  __/ || (__| | | |  / __/ | |_| |
|____/ \__,_|___/\___| |_|  \___|\__\___|_| |_| |_____(_)___/
</pre>

# Keys
<pre>
 - {username}
 - {distro}
 - {hostname}
 - {kernel}
 - {shell}
 - {uptime}
 - {wm}
 - {used_mem}
 - {total_mem}
 - {available_mem}
 - {cpu}
 - {gpu}
 - {packages}
</pre>