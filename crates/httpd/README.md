# hlstate-httpd

Web server implementation based on the Rocket engine

> [!IMPORTANT]
> * IPv6-only servers implementation, make sure `xash3d-query` ([IPv6](https://github.com/YGGverse/xash3d-master/tree/ip6-only/query)) is installed!

## Run

``` bash
cd crates/httpd
cargo run -- -c config.toml
```

## systemd

``` bash
sudo useradd -s /usr/sbin/nologin -Mr hlstate

sudo cp config.toml /etc/hlstate-httpd.toml

mkdir -p /var/www/hlstate-httpd
chown hlstate:hlstate /var/www/hlstate-httpd

mkdir -p /var/log/hlstate-httpd
chown hlstate:hlstate /var/log/hlstate-httpd
```

``` /etc/systemd/system/hlstate-httpd.service
#/etc/systemd/system/hlstate-httpd.service

[Unit]
After=network.target
Wants=network.target

[Service]
Type=simple

User=hlstate
Group=hlstate

# contains templates dir
WorkingDirectory=/var/www/hlstate-httpd

ExecStart=/usr/local/bin/hlstate-httpd -c /etc/hlstate-httpd.toml

# https://github.com/rwf2/Rocket/issues/2951
StandardOutput=null
StandardError=file:/var/log/hlstate-httpd/error.log

[Install]
WantedBy=multi-user.target
```

## nginx

``` /etc/nginx/sites-available/default
#/etc/nginx/sites-available/default

server {
	listen [202:68d0:f0d5:b88d:1d1a:555e:2f6b:3148]:27080;
	server_name hl.ygg ygg.hl.srv;

	access_log /var/log/nginx/hlstate-httpd.access.log;
	error_log /var/log/nginx/hlstate-httpd.error.log warn;

	location / {
		expires 1m;
		add_header Cache-Control "public, max-age=60";
		proxy_pass http://[::1]:8027;
		proxy_set_header X-Real-IP $remote_addr;
		proxy_set_header X-Forwarded-Proto $scheme;
	}
}
```