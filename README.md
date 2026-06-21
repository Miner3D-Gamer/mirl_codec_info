# Mirl Codec Info (0.0.0-alpha)

#### Mici - Parse and Marshal some codecs while retaining metadata about the parsed values

<details>
<summary>Flags</summary>

### Default:

**Core**

- ~~`std` (Default)~~ - `std` is required
- `c_compatible`

**Codec**

- `all_codecs`
- `serde`
- `bitcode`
- `wincode` (bitcode recommended)
- `zerocopy`
- `compactly`

**Enum**

- `all_enum_extensions`
- `strum`
- `enum_ext`

### Custom:

- `preserve_entries` - Inside objects/maps/dictionaries, retain value order and duplicate values

</details>

### Purpose

Instead of solely focusing on speed, retain additional metadata when processing elements in codec formats (like json).

#### Parse

> Parse codecs but retain metadata about them, text -> values

- Json
- CSS

#### Marshal

> Marshal values normally, values -> text

- Json
- CSS
