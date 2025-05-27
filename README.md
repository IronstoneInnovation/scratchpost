![a wide-eyed grey shorthair Scottish fold cat destroying a scratching post](scratchpost_logo.png "scratchpost")

# scratchpost

_A simple key/value cache server built in Rust._

## Installation

Install scratchpost with cargo:

```bash
$ cargo install scratchpost
```

## Settings

There's only one setting available currently, the maximum size of the cache.  This is controlled by setting the SCRATCHPOST_MAX_ITEMS environment variable; if this is not set the default value of 1000 is used.

## Usage

Start scratchpost:

```bash
$ scratchpost
```

When launch Rocket will print the following (your port number may differ):

```
Rocket has launched from http://127.0.0.1:8000
```

### Endpoints

#### `GET /`

Returns http status 200 and the message "Feed me!".

#### `POST /item`

Stores an item (key/value pair) in the cache.  The item is included in the request `body` as a JSON object, like this:

```json
{
    "key": "string",
    "value": "string"
}
```

Note that an empty string value will not be accepted and status 400 will be returned.

If successful, http status 200 is returned.

#### `GET /item/<key>`

Retrieves the value of an item from the cache.  If the item does not exist an empty string is returned.

## License

MIT