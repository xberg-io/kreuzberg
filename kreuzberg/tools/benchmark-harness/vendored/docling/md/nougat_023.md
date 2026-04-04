## Discussion topics, Linux IPsec Workshop

Steffen Klassert secunet Security Networks AG

Dresden

Linux IPsec Workshop, Dresden, March 26, 2018

Discussion topics, Linux IPsec Workshop

Future of PFKEY in the kernel

Configurable system policy default (allow/drop)

Crypto layer problems

Hardware GRO

## Future of PFKEY in the kernel

- glyph[trianglerightsld] PFKEY is buggy
- glyph[trianglerightsld] Google syscall fuzzer reports more and more (security related) bugs
- glyph[trianglerightsld] No active development since more that 10 years
- glyph[trianglerightsld] Do we still need to support PFKEY, and if yes how long?
- glyph[trianglerightsld] What do we need to do to be able to remove PKKEY from the kernel?
- glyph[trianglerightsld] How do we handle the PFKEY bug reports until we can remove it?

## Future of PFKEY in the kernel

## glyph[trianglerightsld] PFKEY is buggy

- glyph[trianglerightsld] Google syscall fuzzer reports more and more (security related) bugs
- glyph[trianglerightsld] No active development since more that 10 years
- glyph[trianglerightsld] Do we still need to support PFKEY, and if yes how long?
- glyph[trianglerightsld] What do we need to do to be able to remove PKKEY from the kernel?
- glyph[trianglerightsld] How do we handle the PFKEY bug reports until we can remove it?

## Future of PFKEY in the kernel

- glyph[trianglerightsld] PFKEY is buggy
- glyph[trianglerightsld] Google syscall fuzzer reports more and more (security related) bugs
- glyph[trianglerightsld] No active development since more that 10 years
- glyph[trianglerightsld] Do we still need to support PFKEY, and if yes how long?
- glyph[trianglerightsld] What do we need to do to be able to remove PKKEY from the kernel?
- glyph[trianglerightsld] How do we handle the PFKEY bug reports until we can remove it?

## Future of PFKEY in the kernel

- glyph[trianglerightsld] PFKEY is buggy
- glyph[trianglerightsld] Google syscall fuzzer reports more and more (security related) bugs
- glyph[trianglerightsld] No active development since more that 10 years
- glyph[trianglerightsld] Do we still need to support PFKEY, and if yes how long?
- glyph[trianglerightsld] What do we need to do to be able to remove PKKEY from the kernel?
- glyph[trianglerightsld] How do we handle the PFKEY bug reports until we can remove it?

## Future of PFKEY in the kernel

- glyph[trianglerightsld] PFKEY is buggy
- glyph[trianglerightsld] Google syscall fuzzer reports more and more (security related) bugs
- glyph[trianglerightsld] No active development since more that 10 years
- glyph[trianglerightsld] Do we still need to support PFKEY, and if yes how long?
- glyph[trianglerightsld] What do we need to do to be able to remove PKKEY from the kernel?
- glyph[trianglerightsld] How do we handle the PFKEY bug reports until we can remove it?

## Future of PFKEY in the kernel

- glyph[trianglerightsld] PFKEY is buggy
- glyph[trianglerightsld] Google syscall fuzzer reports more and more (security related) bugs
- glyph[trianglerightsld] No active development since more that 10 years
- glyph[trianglerightsld] Do we still need to support PFKEY, and if yes how long?
- glyph[trianglerightsld] What do we need to do to be able to remove PKKEY from the kernel?
- glyph[trianglerightsld] How do we handle the PFKEY bug reports until we can remove it?

## Future of PFKEY in the kernel

- glyph[trianglerightsld] PFKEY is buggy
- glyph[trianglerightsld] Google syscall fuzzer reports more and more (security related) bugs
- glyph[trianglerightsld] No active development since more that 10 years
- glyph[trianglerightsld] Do we still need to support PFKEY, and if yes how long?
- glyph[trianglerightsld] What do we need to do to be able to remove PKKEY from the kernel?
- glyph[trianglerightsld] How do we handle the PFKEY bug reports until we can remove it?

## Configurable system policy default (allow/drop)

- glyph[trianglerightsld] The current default behaviour is to allow traffic if there is no matching policy
- glyph[trianglerightsld] A patch that make the default configurable (allow/drop) exists
- glyph[trianglerightsld] Each direction can be configured sepatately (input/output/forward)
- glyph[trianglerightsld] When default is block, we need allow policies for all packet flows we accept
- glyph[trianglerightsld] Would this be usefull for the userspace?

## Configurable system policy default (allow/drop)

- glyph[trianglerightsld] The current default behaviour is to allow traffic if there is no matching policy
- glyph[trianglerightsld] A patch that make the default configurable (allow/drop) exists
- glyph[trianglerightsld] Each direction can be configured sepatately (input/output/forward)
- glyph[trianglerightsld] When default is block, we need allow policies for all packet flows we accept
- glyph[trianglerightsld] Would this be usefull for the userspace?

## Configurable system policy default (allow/drop)

- glyph[trianglerightsld] The current default behaviour is to allow traffic if there is no matching policy
- glyph[trianglerightsld] A patch that make the default configurable (allow/drop) exists
- glyph[trianglerightsld] Each direction can be configured sepatately (input/output/forward)
- glyph[trianglerightsld] When default is block, we need allow policies for all packet flows we accept
- glyph[trianglerightsld] Would this be usefull for the userspace?

## Configurable system policy default (allow/drop)

- glyph[trianglerightsld] The current default behaviour is to allow traffic if there is no matching policy
- glyph[trianglerightsld] A patch that make the default configurable (allow/drop) exists
- glyph[trianglerightsld] Each direction can be configured sepatately (input/output/forward)
- glyph[trianglerightsld] When default is block, we need allow policies for all packet flows we accept
- glyph[trianglerightsld] Would this be usefull for the userspace?

## Configurable system policy default (allow/drop)

- glyph[trianglerightsld] The current default behaviour is to allow traffic if there is no matching policy
- glyph[trianglerightsld] A patch that make the default configurable (allow/drop) exists
- glyph[trianglerightsld] Each direction can be configured sepatately (input/output/forward)
- glyph[trianglerightsld] When default is block, we need allow policies for all packet flows we accept
- glyph[trianglerightsld] Would this be usefull for the userspace?

## Configurable system policy default (allow/drop)

- glyph[trianglerightsld] The current default behaviour is to allow traffic if there is no matching policy
- glyph[trianglerightsld] A patch that make the default configurable (allow/drop) exists
- glyph[trianglerightsld] Each direction can be configured sepatately (input/output/forward)
- glyph[trianglerightsld] When default is block, we need allow policies for all packet flows we accept
- glyph[trianglerightsld] Would this be usefull for the userspace?

## Crypto layer problems

- glyph[trianglerightsld] There is a lot of memcpy in the crypto layer
- glyph[trianglerightsld] IV generators copy if src and dst buffer are different
- glyph[trianglerightsld] Some algorithm implementations are not able to do SG operations
- glyph[trianglerightsld] Might be worth to do some performance optimizations in the crypto layer
- glyph[trianglerightsld] IPsec performance optimizations are 'eaten up' in the crypto layer

## Crypto layer problems

## glyph[trianglerightsld] There is a lot of memcpy in the crypto layer

- glyph[trianglerightsld] IV generators copy if src and dst buffer are different
- glyph[trianglerightsld] Some algorithm implementations are not able to do SG operations
- glyph[trianglerightsld] Might be worth to do some performance optimizations in the crypto layer
- glyph[trianglerightsld] IPsec performance optimizations are 'eaten up' in the crypto layer

## Crypto layer problems

- glyph[trianglerightsld] There is a lot of memcpy in the crypto layer
- glyph[trianglerightsld] IV generators copy if src and dst buffer are different
- glyph[trianglerightsld] Some algorithm implementations are not able to do SG operations
- glyph[trianglerightsld] Might be worth to do some performance optimizations in the crypto layer
- glyph[trianglerightsld] IPsec performance optimizations are 'eaten up' in the crypto layer

## Crypto layer problems

- glyph[trianglerightsld] There is a lot of memcpy in the crypto layer
- glyph[trianglerightsld] IV generators copy if src and dst buffer are different
- glyph[trianglerightsld] Some algorithm implementations are not able to do SG operations
- glyph[trianglerightsld] Might be worth to do some performance optimizations in the crypto layer
- glyph[trianglerightsld] IPsec performance optimizations are 'eaten up' in the crypto layer

## Crypto layer problems

- glyph[trianglerightsld] There is a lot of memcpy in the crypto layer
- glyph[trianglerightsld] IV generators copy if src and dst buffer are different
- glyph[trianglerightsld] Some algorithm implementations are not able to do SG operations
- glyph[trianglerightsld] Might be worth to do some performance optimizations in the crypto layer
- glyph[trianglerightsld] IPsec performance optimizations are 'eaten up' in the crypto layer

## Crypto layer problems

- glyph[trianglerightsld] There is a lot of memcpy in the crypto layer
- glyph[trianglerightsld] IV generators copy if src and dst buffer are different
- glyph[trianglerightsld] Some algorithm implementations are not able to do SG operations
- glyph[trianglerightsld] Might be worth to do some performance optimizations in the crypto layer
- glyph[trianglerightsld] IPsec performance optimizations are 'eaten up' in the crypto layer

Discussion topics, Linux IPsec Workshop Hardware GRO

## Hardware GRO

- glyph[trianglerightsld] Hardware GRO: Routeable version of LRO
- glyph[trianglerightsld] Middleboxes could benefit from receive side HW offload too
- glyph[trianglerightsld] Infrastructure was introduced recently
- glyph[trianglerightsld] Do the NIC vendors plan to support it???

Discussion topics, Linux IPsec Workshop Hardware GRO

## Hardware GRO

- glyph[trianglerightsld] Hardware GRO: Routeable version of LRO
- glyph[trianglerightsld] Middleboxes could benefit from receive side HW offload too
- glyph[trianglerightsld] Infrastructure was introduced recently
- glyph[trianglerightsld] Do the NIC vendors plan to support it???

## Hardware GRO

- glyph[trianglerightsld] Hardware GRO: Routeable version of LRO
- glyph[trianglerightsld] Middleboxes could benefit from receive side HW offload too
- glyph[trianglerightsld] Infrastructure was introduced recently
- glyph[trianglerightsld] Do the NIC vendors plan to support it???

## Hardware GRO

- glyph[trianglerightsld] Hardware GRO: Routeable version of LRO
- glyph[trianglerightsld] Middleboxes could benefit from receive side HW offload too
- glyph[trianglerightsld] Infrastructure was introduced recently
- glyph[trianglerightsld] Do the NIC vendors plan to support it???

Discussion topics, Linux IPsec Workshop

Hardware GRO

## Hardware GRO

- glyph[trianglerightsld] Hardware GRO: Routeable version of LRO
- glyph[trianglerightsld] Middleboxes could benefit from receive side HW offload too
- glyph[trianglerightsld] Infrastructure was introduced recently
- glyph[trianglerightsld] Do the NIC vendors plan to support it???
