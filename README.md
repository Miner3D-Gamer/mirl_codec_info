# Mirl Codec Info (0.0.0-alpha)

#### Mici - Parse and Marshal some codecs while retaining metadata about the parsed values

<details>
<summary>Flags</summary>

### Default:

**Core**

- ~~`std` (Default)~~ - `std` is required

- `c_compatible`

**Codec**

- `serde`
- `bitcode`
- `wincode` (bitcode recommended)

**Enum**

- `strum`
- `enum_ext`

### Custom:

- `preserve_entries` - Inside objects/maps/dictionaries, retain value order and duplicate values

</details>

### Purpose

Retain metadata when processing formats

#### Parse

> Parse codecs but retain metadata about them, text -> values

- Json
- CSS

#### Marshal

> Marshal values normally, values -> text

- Json
- CSS
