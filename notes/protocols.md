# the communication protocols by which hubs communicate

## terms
`client` = local hub  
`remote` = some other hub  

## tracking stuff

### ownership declaration
```
$client:'have:<long hash>,<meta>.'
$remote:'ok.'
```

### mote search
```
$client:'have?:<long hash>,<meta>.'
$remote:'no.'
```

```
$client:'have?:<long hash>,<meta>.'
$remote:'yes.'
```

```
$client:'have?:<long hash>,<meta>.'
$remote:'others:[<remote spec>, ..].'
```

### ownership update
```
$client:'new?:<datetime spec>,count.'
$remote:'ok:[[<long hash>,<meta>], ..].'
```

```
$client:'new?:<datetime spec>,count.'
$remote:'no.'
```

## bootstrapper stuff

### registration
```
$client:'hi?.'
$remote:'ok.'
```

### hub discovery
```
$client:'others?'
$remote:'ok:[<remote spec>, ..].'
```

```
$client:'others?'
$remote:'no.'
```

## pushing
```
$client:'want?:<long hash>,<meta>.'
$remote:'no.'
```

```
$client:'want?:<long hash>,<meta>.'
$remote:'ok.'
```

```
$client:'take:{<mote json>}.'
$remote:'ok.'
```

## pulling
```
$client:'get?:<long hash>,<meta>.'
$remote:'no.'
```

```
$client:'get?:<long hash>,<meta>.'
$remote:'ok:{<mote json>}.'
```

## serving
```
$client:'fetch?:<long hash>.'
$remote:'no.'
```

```
$client:'fetch?:<long hash>.'
$remote:'ok:{<mote json>}.'
```

```
$client:'fetch?:<long hash>.'
$remote:'others:[<remote spec>, ..].'
```
