# URL-matic
Perform some useful manipulation of URLs. This thing doesn't do much yet.

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
