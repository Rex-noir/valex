# valex

Local PHP development server — manages nginx + PHP-FPM + dnsmasq for `.test`
domains, with driver-based project serving and reverse proxy support.

## Usage

```
valex <COMMAND>
```

### Serve

Explicit driver selection with driver-specific options:

```
valex serve laravel --php-version 8.3 [/path/to/project] [domain]
```

Available drivers:

| Driver    | Required flags              |
|-----------|-----------------------------|
| `laravel` | `--php-version <VERSION>`   |

### Proxy

Reverse proxy to an upstream via nginx:

```
valex proxy <domain> <proxy-pass>
valex proxy myapp.test http://localhost:3000
```

Supports WebSocket upgrades (HMR) out of the box.

### Remove

```
valex unserve <domain>
valex unproxy <domain>
```

Deletes the nginx config for the domain and reloads nginx.

### Setup

```
valex setup [Dns | Nginx | PHPFpm]
```

Runs all setup tasks, or a specific one.

### Status

```
valex status
```

Shows `systemctl is-active` for nginx, dnsmasq, and all configured PHP-FPM
services.

### Restart

```
valex restart
```

Restarts nginx, dnsmasq, and all configured PHP-FPM services.

### Elevate

```
valex elevate
```

Writes a sudoers file at `/etc/sudoers.d/valex` to grant passwordless sudo
for the `systemctl` commands valex needs.

### Completions

```
valex completions bash
valex completions zsh
valex completions fish
valex completions powershell
valex completions elvish
```

## Configuration

Config file at `~/.config/valex/config.json5`:

```json5
{
  php: {
    "8.4": {
      fpm_config_path: "/etc/php/8.4/fpm/php-fpm.conf",
      fpm_socket_path: "/var/run/php/php8.4-fpm.sock",
      fpm_service_name: "php8.4-fpm",
    },
  },
}
```

## SELinux

On distros with SELinux enabled, allow nginx to read user content:

```
sudo setsebool -P httpd_read_user_content 1
```
