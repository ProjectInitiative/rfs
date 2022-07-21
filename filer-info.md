You can append to any HTTP API with &pretty=y to see a formatted json output.
## Filer server
### POST/PUT/GET files
```bash
# Basic Usage:
  //create or overwrite the file, the directories /path/to will be automatically created
  POST /path/to/file
  PUT /path/to/file
  //create or overwrite the file, the filename in the multipart request will be used
  POST /path/to/
  //create or append the file
  POST /path/to/file?op=append
  PUT /path/to/file?op=append
  //get the file content
  GET /path/to/file

  //return a json format subdirectory and files listing
  GET /path/to/
  Accept: application/json

# options for POST a file:
  // set file TTL
  POST /path/to/file?ttl=1d
  // set file mode when creating or overwriting a file
  POST /path/to/file?mode=0755
```
| POST/PUT Parameter | Description | Default |
| ---- | -- | -- |
| dataCenter | data center | empty |
| rack | rack | empty |
| dataNode | data node | empty |
| collection | collection | empty |
| replication | replication | empty |
| fsync | if "true", the file content write will incur an fsync operation (though the file metadata will still be separate) | false |
| ttl | time to live, examples, 3m: 3 minutes, 4h: 4 hours, 5d: 5 days, 6w: 6 weeks, 7M: 7 months, 8y: 8 years | empty |
| maxMB | max chunk size | empty |
| mode | file mode | 0660 |
| op | file operation, currently only support "append" | empty |
| skipCheckParentDir | Ensuring parent directory exists cost one metadata API call. Skipping this can reduce network latency. | false |
| header: `Content-Type` | used for auto compression | empty |
| header: `Content-Disposition` | used as response content-disposition | empty |
| prefixed header: `Seaweed-` | example: `Seaweed-name1: value1`. Returned as `Seaweed-Name1: value1` in GET/HEAD response header. | empty |

| GET Parameter | Description | Default |
| ---- | -- | -- |
| metadata | get file metadata | false |
| resolveManifest | resolve manifest chunks | false |
### notice
* It is recommended to add retries when writing to Filer.
* `AutoChunking` is not supported for method `PUT`. If the file length is greater than 256MB, only the leading 256MB in the `PUT` request will be saved.
* When appending to a file, each append will create one chunk and added to the file metadata. If there are too many small appends, there could be too many chunks. So try to keep each append size reasonably big.

Examples:
```bash
# Basic Usage:
> curl -F file=@report.js "http://localhost:8888/javascript/"
{"name":"report.js","size":866,"fid":"7,0254f1f3fd","url":"http://localhost:8081/7,0254f1f3fd"}
> curl  "http://localhost:8888/javascript/report.js"   # get the file content
> curl -I "http://localhost:8888/javascript/report.js" # get only header
...
> curl -F file=@report.js "http://localhost:8888/javascript/new_name.js"    # upload the file to a different name
{"name":"report.js","size":5514}
> curl -T test.yaml http://localhost:8888/test.yaml                         # upload file by PUT
{"name":"test.yaml","size":866}
> curl -F file=@report.js "http://localhost:8888/javascript/new_name.js?op=append"    # append to an file
{"name":"report.js","size":5514}
> curl -T test.yaml http://localhost:8888/test.yaml?op=append                         # append to an file by PUT
{"name":"test.yaml","size":866}
> curl -H "Accept: application/json" "http://localhost:8888/javascript/?pretty=y"            # list all files under /javascript/
{
  "Path": "/javascript",
  "Entries": [
    {
      "FullPath": "/javascript/jquery-2.1.3.min.js",
      "Mtime": "2020-04-19T16:08:14-07:00",
      "Crtime": "2020-04-19T16:08:14-07:00",
      "Mode": 420,
      "Uid": 502,
      "Gid": 20,
      "Mime": "text/plain; charset=utf-8",
      "Replication": "000",
      "Collection": "",
      "TtlSec": 0,
      "UserName": "",
      "GroupNames": null,
      "SymlinkTarget": "",
      "Md5": null,
      "Extended": null,
      "chunks": [
        {
          "file_id": "2,087f23051201",
          "size": 84320,
          "mtime": 1587337694775717000,
          "e_tag": "32015dd42e9582a80a84736f5d9a44d7",
          "fid": {
            "volume_id": 2,
            "file_key": 2175,
            "cookie": 587534849
          },
          "is_gzipped": true
        }
      ]
    },
    {
      "FullPath": "/javascript/jquery-sparklines",
      "Mtime": "2020-04-19T16:08:14-07:00",
      "Crtime": "2020-04-19T16:08:14-07:00",
      "Mode": 2147484152,
      "Uid": 502,
      "Gid": 20,
      "Mime": "",
      "Replication": "000",
      "Collection": "",
      "TtlSec": 0,
      "UserName": "",
      "GroupNames": null,
      "SymlinkTarget": "",
      "Md5": null,
      "Extended": null
    }
  ],
  "Limit": 100,
  "LastFileName": "jquery-sparklines",
  "ShouldDisplayLoadMore": false
}
# get file metadata
> curl 'http://localhost:8888/test01.py?metadata=true&pretty=yes'
{
  "FullPath": "/test01.py",
  "Mtime": "2022-01-09T19:11:18+08:00",
  "Crtime": "2022-01-09T19:11:18+08:00",
  "Mode": 432,
  "Uid": 1001,
  "Gid": 1001,
  "Mime": "text/x-python",
  "Replication": "",
  "Collection": "",
  "TtlSec": 0,
  "DiskType": "",
  "UserName": "",
  "GroupNames": null,
  "SymlinkTarget": "",
  "Md5": "px6as5eP7tF5YcgAv5m60Q==",
  "FileSize": 1992,
  "Extended": null,
  "chunks": [
    {
      "file_id": "17,04fbb55507b515",
      "size": 1992,
      "mtime": 1641726678984876713,
      "e_tag": "px6as5eP7tF5YcgAv5m60Q==",
      "fid": {
        "volume_id": 17,
        "file_key": 326581,
        "cookie": 1426568469
      },
      "is_compressed": true
    }
  ],
  "HardLinkId": null,
  "HardLinkCounter": 0,
  "Content": null,
  "Remote": null,
  "Quota": 0
}
```
### GET files

```bash
  //get file with a different content-disposition
  GET /path/to/file?response-content-disposition=attachment%3B%20filename%3Dtesting.txt
```
| GET Parameter | Description | Default |
| ---- | -- | -- |
| response-content-disposition | used as response content-disposition | empty |


### PUT/DELETE file tagging
```
# put 2 pairs of meta data
curl -X PUT -H "Seaweed-Name1: value1" -H "Seaweed-some: some string value" http://localhost:8888/path/to/a/file?tagging
# read the meta data from HEAD request
curl -I "http://localhost:8888/path/to/a/file"
...
Seaweed-Name1: value1
Seaweed-Some: some string value
...
# delete all "Seaweed-" prefixed meta data
curl -X DELETE http://localhost:8888/path/to/a/file?tagging
# delete specific "Seaweed-" prefixed meta data
curl -X DELETE http://localhost:8888/path/to/a/file?tagging=Name1,Some

```
| Method | Request | Header | Operation |
| ---- | ---- | -- | -- |
| PUT | <file_url>?tagging | Prefixed with "Seaweed-" | set the meta data  |
| DELETE | <file_url>?tagging |  | remove all the "Seaweed-" prefixed header  |
| DELETE | <file_url>?tagging=Some,Name |  | remove the headers "Seaweed-Some", "Seaweed-Name" |

Notice that the tag names follow http header key convention, with the first character capitalized.

### Move files and directories
```bash
# move(rename) "/path/to/src_file" to "/path/to/dst_file"
> curl -X POST 'http://localhost:8888/path/to/dst_file?mv.from=/path/to/src_file'
```
| POST Parameter | Description | Default |
| ---- | -- | -- |
| mv.from | move from one file or directory to another location | Required field |

### Create an empty folder
Folders usually are created automatically when uploading a file. To create an empty file, you can use this:
```
curl -X POST "http://localhost:8888/test/"
```
### List files under a directory
Some folder can be very large. To efficiently list files, we use a non-traditional way to iterate files. Every pagination you provide a "lastFileName", and a "limit=x". The filer locate the "lastFileName" in O(log(n)) time, and retrieve the next x files.
```bash
curl -H "Accept: application/json" "http://localhost:8888/javascript/?pretty=y&lastFileName=jquery-2.1.3.min.js&limit=2"
{
  "Path": "/javascript",
  "Entries": [
    {
      "FullPath": "/javascript/jquery-sparklines",
      "Mtime": "2020-04-19T16:08:14-07:00",
      "Crtime": "2020-04-19T16:08:14-07:00",
      "Mode": 2147484152,
      "Uid": 502,
      "Gid": 20,
      "Mime": "",
      "Replication": "000",
      "Collection": "",
      "TtlSec": 0,
      "UserName": "",
      "GroupNames": null,
      "SymlinkTarget": "",
      "Md5": null,
      "Extended": null
    }
  ],
  "Limit": 2,
  "LastFileName": "jquery-sparklines",
  "ShouldDisplayLoadMore": false
}
```
| Parameter | Description | Default |
| ---- | -- | -- |
| limit | how many file to show | 100 |
| lastFileName | the last file in previous batch | empty |
| namePattern | match file names, case-sensitive wildcard characters '*' and '?' | empty |
| namePatternExclude | nagetive match file names, case-sensitive wildcard characters '*' and '?' | empty |

## Supported Name Patterns
The patterns are case-sensitive and support wildcard characters '*' and '?'.
| Pattern | Matches |
| ---- | -- |
| * | any file name |
| *.jpg | abc.jpg |
| a*.jp*g | abc.jpg, abc.jpeg |
| a*.jp?g | abc.jpeg |
# Deletion
## Delete a file
```bash
> curl -X DELETE http://localhost:8888/path/to/file
```
## Delete a folder
```bash
// recursively delete all files and folders under a path
> curl -X DELETE http://localhost:8888/path/to/dir?recursive=true
// recursively delete everything, ignoring any recursive error
> curl -X DELETE http://localhost:8888/path/to/dir?recursive=true&ignoreRecursiveError=true
// For Experts Only: remove filer directories only, without removing data chunks. 
// see https://github.com/chrislusf/seaweedfs/pull/1153
> curl -X DELETE http://localhost:8888/path/to?recursive=true&skipChunkDeletion=true
```
| Parameter | Description | Default |
| ---- | -- | -- |
| recursive | if "recursive=true", recursively delete all files and folders | filer recursive_delete option from filer.toml |
| ignoreRecursiveError | if "ignoreRecursiveError=true", ignore errors in recursive mode | false |
| skipChunkDeletion | if "skipChunkDeletion=true", do not delete file chunks on volume servers | false |