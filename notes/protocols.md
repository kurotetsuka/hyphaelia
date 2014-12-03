# the communication protocols by which hubs communicate

## terms
client = local hub
daemon = remote hub

## tracking stuff

### ownership declaration
```
$client:'have:<long hash>,<meta>.'
$daemon:'ok.'
```

### mote search
```
$client:'have?:<long hash>.'
$daemon:'no.'
```

```
$client:'have?:<long hash>.'
$daemon:'yes.'
```

```
$client:'have?:<long hash>.'
$daemon:'other:<ipaddr/hostname>:<port>.'
```

### ownership update
```
$client:'new?:<datetime spec>'
$daemon:'ok:[[<long hash>,<meta>],..]'
```

## bootstrapper stuff

### registration
```
$client:'hi?'
$daemon:'hi.'
```

### hub discovery
```
$client:'others?'
$daemon:'ok:[<remote spec>, .. ].'
```

```
$client:'others?'
$daemon:'no.'
```

## pushing
```
$client:'want?:<long hash>,<meta>.'
$daemon:'no.'
```

```
$client:'want?:<long hash>,<meta>.'
$daemon:'ok.'
$client:'ok:{<mote json>}.'
```

## pulling
```
$client:'get?:<long hash>.'
$daemon:'no.'
```

```
$client:'get?:<long hash>'
$daemon:'ok:{<mote json>}.'
```

## serving
```
$client:'fetch?:<long hash>.'
$daemon:'no.'
```

```
$client:'fetch?:<long hash>.'
$daemon:'ok:{<mote json>}.'
```

```
$client:'fetch?:<long hash>.'
$daemon:'ok:{<mote json>}.'
```
