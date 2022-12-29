# The Problem

There is no consumer grade HA, hyper-converged storage solution that meets enterprise quality of service on mismatched user hardware.
Think FOSS geo-distributed Microsoft Storage Spaces with consumer hardware (SATA SSDs and SMR/CMR HDDs) of various size.

Some examples are:

* [Seaweedfs](https://github.com/seaweedfs/)
  * Doesn't have a stable mount for distributed kvm-qemu VM support
* [Garage](https://garagehq.deuxfleurs.fr/)
  * Doesn't have a direct POSIX compliant filesystem, and is a non-goal
	* S3fuse style mounts don't provide the POSIX compliance needed for kvm-qemu VMs
	* Doesn't support Erasure Coding (ec) for mismatched harddrives
* [tifs](https://github.com/Hexilee/tifs)
	* Good concept and implmentation
	* TiKV does not work well on HDDs, rendering tifs useless on any HDD setup

## Ideas

* Use rclone's S3 VFS APIs to bridge the gap between POSIX and S3fuse solution
This did not pan out, as the caching still required downloading the fully modified block per object.

* Setup garage and use [s3backer](https://github.com/archiecobbs/s3backer) to provide raw block devices for kvm-qemu VMs
This in theory can work. Proxmox utilizes [rbd](https://docs.ceph.com/en/quincy/rbd/) to achieve block level devices for VMs. Qemu had direct support to connect to a ceph [rbd](https://github.com/qemu/qemu/blob/master/block/rbd.c), as well as [qemu-img](https://docs.ceph.com/en/quincy/rbd/qemu-rbd/). This is acheived because of the librbd library.
Again in theory we could get a working setup with S3 and s3backer. It would require large amounts of work: qemu support via a library, direct integration and so on.
Another potential solution is to add a less direct way to support this by creating scripts or a Rust binary to support Proxmox storage scripts, via mounting and umount s3backer block devices to the host during a live migration, and passing through those disks to the VM instead of relying on kvm-qemu to connect to the block device through s3backer directly.

* Dissect the Garage codebase, fork and build a FS API on top of the S3 API

* Write new distributed FS similar to a hybrid seaweedfs (volume server architecture) and garage (non-raft architecture)
