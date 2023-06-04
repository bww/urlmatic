# URL-matic
Perform some useful manipulation of URLs, such as:

* Resolve a URL or path against a base,
* Trim path components from the end of a URL,
* Rewrite a URL by replacing the host, path, query, fragment, etc,
* Encode and decode query strings.

## Installing

You can install _URL-matic_ via [Homebrew](https://brew.sh/) on macOS as follows:

```
$ brew install bww/stable/urlmatic 
```

If you have a Rust toolchain installed, you can also install from Cargo:

```
$ cargo install urlmatic
```

### Resolve a URL Against a Base
Resolves a relative URL against the provided base URL and prints the result.

```
$ urlmatic resolve --base 'https://www.example.com/documents/letter.html?length=100' '../index.html?length=200'
> https://www.example.com/index.html?length=200
```

### Trim Path Components from a URL
Removes the specified number of components from the end of a URL's path and prints the result.

```
$ urlmatic trim --count 2 'https://www.example.com/documents/letter.html?length=100'
> https://www.example.com/?length=100
```

### Rewrite a URL by Replacing Components
Replace specific components in a URL and print the result.

```
$ urlmatic rewrite \
    --username admin \
    --host another.com \
    --path /cgi-bin/q \
    --query 'offset=0&length=100' \
    --fragment 'anchor-name' \
    https://example.com/query
> https://admin@another.com/cgi-bin/q?offset=0&length=100#anchor-name
```

### Encode a Query String as `x-www-form-urlencoded` data
URL-encode form data form/query data.

```
$ urlmatic encode -k yep -v 👍 -k nope -v 👎
> yep=%F0%9F%91%8D&nope=%F0%9F%91%8E
```
```
$ urlmatic encode yep=👍 nope=👎
> nope=%F0%9F%91%8E&yep=%F0%9F%91%8D
```

### Decode a Query String as `x-www-form-urlencoded` data
URL-decode form data and extract values.

```
$ urlmatic decode 'yep=%F0%9F%91%8D&nope=%F0%9F%91%8E'
>  yep: 👍
> nope: 👎
```
```
$ urlmatic decode --select yep 'yep=%F0%9F%91%8D&nope=%F0%9F%91%8E'
> 👍
```
```
$ urlmatic decode --select nope,yep 'yep=%F0%9F%91%8D&nope=%F0%9F%91%8E'
> 👍
> 👎
```

### Put it All Together
Compose and modify a URL using a few commands.

```
$ echo 'https://example.com/path/to/query' |
    urlmatic rewrite --host another.com --query $(urlmatic encode -k yep -v 👍) |
    urlmatic trim --count 2
> https://another.com/path?yep=%F0%9F%91%8D
```
