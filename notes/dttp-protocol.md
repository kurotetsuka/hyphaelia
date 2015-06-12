# The protocol by which hubs communicate

## terms
`$client` = local hub  
`$remote` = some other hub  

## tracking stuff

### ownership declaration
```
$client:'have:<mote spec>.'
$remote:'ok.'
```

### mote search
```
$client:'have?:<mote spec>.'
$remote:'no.'
```

```
$client:'have?:<mote spec>.'
$remote:'yes.'
```

```
$client:'have?:<mote spec>.'
$remote:'others:[<remote spec>,.. ].'
```

### ownership update
```
$client:'new?:<datetime spec>,count.'
$remote:'ok:[<mote spec>,.. ].'
```

```
$client:'new?:<datetime spec>,count.'
$remote:'no.'
```

## bootstrapper stuff

### registration
```
$client:'self:<self remote spec>.'
$remote:'ok.'
```

### hub discovery
```
$client:'others?'
$remote:'ok:[<remote spec>,.. ].'
```

```
$client:'others?'
$remote:'no.'
```

## pushing
```
$client:'want?:<mote spec>.'
$remote:'no.'
```

```
$client:'want?:<mote spec>.'
$remote:'yes.'
```

```
$client:'take:{<mote json>}.'
$remote:'ok.'
```

## pulling
```
$client:'get?:<mote spec>.'
$remote:'no.'
```

```
$client:'get?:<mote spec>.'
$remote:'ok:{<mote json>}.'
```

## serving
```
$client:'fetch?:<mote spec>.'
$remote:'no.'
```

```
$client:'fetch?:<mote spec>.'
$remote:'ok:<mote spec>.'
```

```
$client:'fetch?:<mote spec>.'
$remote:'others:[<remote spec>,..].'
```
