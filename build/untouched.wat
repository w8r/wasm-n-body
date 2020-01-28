(module
 (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
 (type $none_=>_none (func))
 (type $i32_=>_none (func (param i32)))
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $none_=>_f64 (func (result f64)))
 (type $i32_i32_i32_=>_none (func (param i32 i32 i32)))
 (type $i32_i32_i32_i32_=>_none (func (param i32 i32 i32 i32)))
 (type $i32_i32_i32_=>_i32 (func (param i32 i32 i32) (result i32)))
 (type $i32_f64_=>_i32 (func (param i32 f64) (result i32)))
 (type $i32_f64_f64_f64_f64_f64_=>_i32 (func (param i32 f64 f64 f64 f64 f64) (result i32)))
 (type $i32_=>_f64 (func (param i32) (result f64)))
 (import "env" "memory" (memory $0 1))
 (data (i32.const 16) "\1c\00\00\00\01\00\00\00\01\00\00\00\1c\00\00\00I\00n\00v\00a\00l\00i\00d\00 \00l\00e\00n\00g\00t\00h\00")
 (data (i32.const 64) "&\00\00\00\01\00\00\00\01\00\00\00&\00\00\00~\00l\00i\00b\00/\00a\00r\00r\00a\00y\00b\00u\00f\00f\00e\00r\00.\00t\00s\00")
 (data (i32.const 128) "\06\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\93 \00\00\02\00\00\00")
 (import "env" "abort" (func $~lib/builtins/abort (param i32 i32 i32 i32)))
 (table $0 1 funcref)
 (global $~lib/rt/stub/startOffset (mut i32) (i32.const 0))
 (global $~lib/rt/stub/offset (mut i32) (i32.const 0))
 (global $~lib/math/NativeMath.PI f64 (f64.const 3.141592653589793))
 (global $assembly/index/SOLAR_MASS f64 (f64.const 39.47841760435743))
 (global $assembly/index/DAYS_PER_YEAR f64 (f64.const 365.24))
 (global $assembly/index/N_BODIES i32 (i32.const 1000))
 (global $assembly/index/system (mut i32) (i32.const 0))
 (global $~lib/ASC_SHRINK_LEVEL i32 (i32.const 0))
 (global $~lib/rt/__rtti_base i32 (i32.const 128))
 (global $~lib/heap/__heap_base i32 (i32.const 180))
 (export "memory" (memory $0))
 (export "__alloc" (func $~lib/rt/stub/__alloc))
 (export "__retain" (func $~lib/rt/stub/__retain))
 (export "__release" (func $~lib/rt/stub/__release))
 (export "__collect" (func $~lib/rt/stub/__collect))
 (export "__reset" (func $~lib/rt/stub/__reset))
 (export "__rtti_base" (global $~lib/rt/__rtti_base))
 (export "init" (func $assembly/index/init))
 (export "step" (func $assembly/index/step))
 (export "e" (func $assembly/index/e))
 (export "bench" (func $assembly/index/bench))
 (export "getBody" (func $assembly/index/getBody))
 (start $~start)
 (func $~lib/rt/stub/maybeGrowMemory (; 1 ;) (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  memory.size
  local.set $1
  local.get $1
  i32.const 16
  i32.shl
  local.set $2
  local.get $0
  local.get $2
  i32.gt_u
  if
   local.get $0
   local.get $2
   i32.sub
   i32.const 65535
   i32.add
   i32.const 65535
   i32.const -1
   i32.xor
   i32.and
   i32.const 16
   i32.shr_u
   local.set $3
   local.get $1
   local.tee $4
   local.get $3
   local.tee $5
   local.get $4
   local.get $5
   i32.gt_s
   select
   local.set $4
   local.get $4
   memory.grow
   i32.const 0
   i32.lt_s
   if
    local.get $3
    memory.grow
    i32.const 0
    i32.lt_s
    if
     unreachable
    end
   end
  end
  local.get $0
  global.set $~lib/rt/stub/offset
 )
 (func $~lib/rt/stub/__alloc (; 2 ;) (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  local.get $0
  i32.const 1073741808
  i32.gt_u
  if
   unreachable
  end
  global.get $~lib/rt/stub/offset
  i32.const 16
  i32.add
  local.set $2
  local.get $0
  i32.const 15
  i32.add
  i32.const 15
  i32.const -1
  i32.xor
  i32.and
  local.tee $3
  i32.const 16
  local.tee $4
  local.get $3
  local.get $4
  i32.gt_u
  select
  local.set $5
  local.get $2
  local.get $5
  i32.add
  call $~lib/rt/stub/maybeGrowMemory
  local.get $2
  i32.const 16
  i32.sub
  local.set $6
  local.get $6
  local.get $5
  i32.store
  local.get $6
  i32.const 1
  i32.store offset=4
  local.get $6
  local.get $1
  i32.store offset=8
  local.get $6
  local.get $0
  i32.store offset=12
  local.get $2
 )
 (func $~lib/rt/stub/__retain (; 3 ;) (param $0 i32) (result i32)
  local.get $0
 )
 (func $~lib/rt/stub/__release (; 4 ;) (param $0 i32)
  nop
 )
 (func $~lib/rt/stub/__collect (; 5 ;)
  nop
 )
 (func $~lib/rt/stub/__reset (; 6 ;)
  global.get $~lib/rt/stub/startOffset
  global.set $~lib/rt/stub/offset
 )
 (func $~lib/memory/memory.fill (; 7 ;) (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i64)
  (local $9 i32)
  block $~lib/util/memory/memset|inlined.0
   local.get $0
   local.set $5
   local.get $1
   local.set $4
   local.get $2
   local.set $3
   local.get $3
   i32.eqz
   if
    br $~lib/util/memory/memset|inlined.0
   end
   local.get $5
   local.get $4
   i32.store8
   local.get $5
   local.get $3
   i32.add
   i32.const 1
   i32.sub
   local.get $4
   i32.store8
   local.get $3
   i32.const 2
   i32.le_u
   if
    br $~lib/util/memory/memset|inlined.0
   end
   local.get $5
   i32.const 1
   i32.add
   local.get $4
   i32.store8
   local.get $5
   i32.const 2
   i32.add
   local.get $4
   i32.store8
   local.get $5
   local.get $3
   i32.add
   i32.const 2
   i32.sub
   local.get $4
   i32.store8
   local.get $5
   local.get $3
   i32.add
   i32.const 3
   i32.sub
   local.get $4
   i32.store8
   local.get $3
   i32.const 6
   i32.le_u
   if
    br $~lib/util/memory/memset|inlined.0
   end
   local.get $5
   i32.const 3
   i32.add
   local.get $4
   i32.store8
   local.get $5
   local.get $3
   i32.add
   i32.const 4
   i32.sub
   local.get $4
   i32.store8
   local.get $3
   i32.const 8
   i32.le_u
   if
    br $~lib/util/memory/memset|inlined.0
   end
   i32.const 0
   local.get $5
   i32.sub
   i32.const 3
   i32.and
   local.set $6
   local.get $5
   local.get $6
   i32.add
   local.set $5
   local.get $3
   local.get $6
   i32.sub
   local.set $3
   local.get $3
   i32.const -4
   i32.and
   local.set $3
   i32.const -1
   i32.const 255
   i32.div_u
   local.get $4
   i32.const 255
   i32.and
   i32.mul
   local.set $7
   local.get $5
   local.get $7
   i32.store
   local.get $5
   local.get $3
   i32.add
   i32.const 4
   i32.sub
   local.get $7
   i32.store
   local.get $3
   i32.const 8
   i32.le_u
   if
    br $~lib/util/memory/memset|inlined.0
   end
   local.get $5
   i32.const 4
   i32.add
   local.get $7
   i32.store
   local.get $5
   i32.const 8
   i32.add
   local.get $7
   i32.store
   local.get $5
   local.get $3
   i32.add
   i32.const 12
   i32.sub
   local.get $7
   i32.store
   local.get $5
   local.get $3
   i32.add
   i32.const 8
   i32.sub
   local.get $7
   i32.store
   local.get $3
   i32.const 24
   i32.le_u
   if
    br $~lib/util/memory/memset|inlined.0
   end
   local.get $5
   i32.const 12
   i32.add
   local.get $7
   i32.store
   local.get $5
   i32.const 16
   i32.add
   local.get $7
   i32.store
   local.get $5
   i32.const 20
   i32.add
   local.get $7
   i32.store
   local.get $5
   i32.const 24
   i32.add
   local.get $7
   i32.store
   local.get $5
   local.get $3
   i32.add
   i32.const 28
   i32.sub
   local.get $7
   i32.store
   local.get $5
   local.get $3
   i32.add
   i32.const 24
   i32.sub
   local.get $7
   i32.store
   local.get $5
   local.get $3
   i32.add
   i32.const 20
   i32.sub
   local.get $7
   i32.store
   local.get $5
   local.get $3
   i32.add
   i32.const 16
   i32.sub
   local.get $7
   i32.store
   i32.const 24
   local.get $5
   i32.const 4
   i32.and
   i32.add
   local.set $6
   local.get $5
   local.get $6
   i32.add
   local.set $5
   local.get $3
   local.get $6
   i32.sub
   local.set $3
   local.get $7
   i64.extend_i32_u
   local.get $7
   i64.extend_i32_u
   i64.const 32
   i64.shl
   i64.or
   local.set $8
   loop $while-continue|0
    local.get $3
    i32.const 32
    i32.ge_u
    local.set $9
    local.get $9
    if
     local.get $5
     local.get $8
     i64.store
     local.get $5
     i32.const 8
     i32.add
     local.get $8
     i64.store
     local.get $5
     i32.const 16
     i32.add
     local.get $8
     i64.store
     local.get $5
     i32.const 24
     i32.add
     local.get $8
     i64.store
     local.get $3
     i32.const 32
     i32.sub
     local.set $3
     local.get $5
     i32.const 32
     i32.add
     local.set $5
     br $while-continue|0
    end
   end
  end
 )
 (func $~lib/arraybuffer/ArrayBufferView#constructor (; 8 ;) (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  local.get $1
  i32.const 1073741808
  local.get $2
  i32.shr_u
  i32.gt_u
  if
   i32.const 32
   i32.const 80
   i32.const 23
   i32.const 56
   call $~lib/builtins/abort
   unreachable
  end
  local.get $1
  local.get $2
  i32.shl
  local.tee $1
  i32.const 0
  call $~lib/rt/stub/__alloc
  local.set $3
  local.get $3
  i32.const 0
  local.get $1
  call $~lib/memory/memory.fill
  local.get $0
  i32.eqz
  if
   i32.const 12
   i32.const 2
   call $~lib/rt/stub/__alloc
   call $~lib/rt/stub/__retain
   local.set $0
  end
  local.get $0
  i32.const 0
  i32.store
  local.get $0
  i32.const 0
  i32.store offset=4
  local.get $0
  i32.const 0
  i32.store offset=8
  local.get $0
  local.tee $4
  local.get $3
  local.tee $5
  local.get $4
  i32.load
  local.tee $6
  i32.ne
  if
   local.get $5
   call $~lib/rt/stub/__retain
   local.set $5
   local.get $6
   call $~lib/rt/stub/__release
  end
  local.get $5
  i32.store
  local.get $0
  local.get $3
  i32.store offset=4
  local.get $0
  local.get $1
  i32.store offset=8
  local.get $0
 )
 (func $~lib/array/Array<assembly/index/Point>#constructor (; 9 ;) (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  if (result i32)
   local.get $0
  else
   i32.const 16
   i32.const 5
   call $~lib/rt/stub/__alloc
   call $~lib/rt/stub/__retain
  end
  local.get $1
  i32.const 2
  call $~lib/arraybuffer/ArrayBufferView#constructor
  local.set $0
  local.get $0
  i32.const 0
  i32.store offset=12
  local.get $0
  local.get $1
  i32.store offset=12
  local.get $0
 )
 (func $assembly/index/Point#constructor (; 10 ;) (param $0 i32) (param $1 f64) (param $2 f64) (param $3 f64) (param $4 f64) (param $5 f64) (result i32)
  local.get $0
  i32.eqz
  if
   i32.const 40
   i32.const 4
   call $~lib/rt/stub/__alloc
   call $~lib/rt/stub/__retain
   local.set $0
  end
  local.get $0
  local.get $1
  f64.store
  local.get $0
  local.get $2
  f64.store offset=8
  local.get $0
  local.get $3
  f64.store offset=16
  local.get $0
  local.get $4
  f64.store offset=24
  local.get $0
  local.get $5
  f64.store offset=32
  local.get $0
 )
 (func $~lib/array/Array<assembly/index/Point>#__unchecked_set (; 11 ;) (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  local.get $2
  call $~lib/rt/stub/__retain
  local.set $2
  local.get $0
  i32.load offset=4
  local.get $1
  i32.const 2
  i32.shl
  i32.add
  local.set $3
  local.get $3
  i32.load
  local.set $4
  local.get $2
  local.get $4
  i32.ne
  if
   local.get $3
   local.get $2
   call $~lib/rt/stub/__retain
   i32.store
   local.get $4
   call $~lib/rt/stub/__release
  end
  local.get $2
  call $~lib/rt/stub/__release
 )
 (func $assembly/index/System#constructor (; 12 ;) (param $0 i32) (param $1 i32) (result i32)
  local.get $1
  call $~lib/rt/stub/__retain
  local.set $1
  local.get $0
  i32.eqz
  if
   i32.const 4
   i32.const 3
   call $~lib/rt/stub/__alloc
   call $~lib/rt/stub/__retain
   local.set $0
  end
  local.get $0
  local.get $1
  call $~lib/rt/stub/__retain
  i32.store
  local.get $1
  call $~lib/rt/stub/__release
  local.get $0
 )
 (func $~lib/array/Array<assembly/index/Point>#get:length (; 13 ;) (param $0 i32) (result i32)
  local.get $0
  i32.load offset=12
 )
 (func $~lib/array/Array<assembly/index/Point>#__unchecked_get (; 14 ;) (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  i32.load offset=4
  local.get $1
  i32.const 2
  i32.shl
  i32.add
  i32.load
  call $~lib/rt/stub/__retain
 )
 (func $assembly/index/System#energy (; 15 ;) (param $0 i32) (result f64)
  (local $1 f64)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 f64)
  (local $9 f64)
  (local $10 f64)
  (local $11 f64)
  (local $12 f64)
  (local $13 i32)
  (local $14 i32)
  (local $15 i32)
  (local $16 i32)
  (local $17 f64)
  (local $18 f64)
  (local $19 f64)
  f64.const 0
  local.set $1
  local.get $0
  i32.load
  call $~lib/rt/stub/__retain
  local.set $2
  i32.const 0
  local.set $3
  local.get $2
  call $~lib/array/Array<assembly/index/Point>#get:length
  local.set $4
  loop $for-loop|0
   local.get $3
   local.get $4
   i32.lt_s
   local.set $5
   local.get $5
   if
    local.get $2
    local.get $3
    call $~lib/array/Array<assembly/index/Point>#__unchecked_get
    local.tee $6
    call $~lib/rt/stub/__retain
    local.set $7
    local.get $7
    f64.load
    local.set $8
    local.get $7
    f64.load offset=8
    local.set $9
    local.get $7
    f64.load offset=16
    local.set $10
    local.get $7
    f64.load offset=24
    local.set $11
    local.get $7
    f64.load offset=32
    local.set $12
    local.get $1
    f64.const 0.5
    local.get $12
    f64.mul
    local.get $10
    local.get $10
    f64.mul
    local.get $11
    local.get $11
    f64.mul
    f64.add
    f64.mul
    f64.add
    local.set $1
    local.get $3
    i32.const 1
    i32.add
    local.set $13
    loop $for-loop|1
     local.get $13
     local.get $4
     i32.lt_s
     local.set $14
     local.get $14
     if
      local.get $2
      local.get $13
      call $~lib/array/Array<assembly/index/Point>#__unchecked_get
      local.tee $15
      call $~lib/rt/stub/__retain
      local.set $16
      local.get $8
      local.get $16
      f64.load
      f64.sub
      local.set $17
      local.get $9
      local.get $16
      f64.load offset=8
      f64.sub
      local.set $18
      local.get $17
      local.get $17
      f64.mul
      local.get $18
      local.get $18
      f64.mul
      f64.add
      local.set $19
      local.get $19
      f64.sqrt
      local.set $19
      local.get $1
      local.get $12
      local.get $16
      f64.load offset=32
      f64.mul
      local.get $19
      f64.div
      f64.sub
      local.set $1
      local.get $15
      call $~lib/rt/stub/__release
      local.get $16
      call $~lib/rt/stub/__release
      local.get $13
      i32.const 1
      i32.add
      local.set $13
      br $for-loop|1
     end
    end
    local.get $6
    call $~lib/rt/stub/__release
    local.get $7
    call $~lib/rt/stub/__release
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|0
   end
  end
  local.get $1
  local.set $12
  local.get $2
  call $~lib/rt/stub/__release
  local.get $12
 )
 (func $assembly/index/init (; 16 ;) (result f64)
  (local $0 i32)
  (local $1 f64)
  (local $2 f64)
  (local $3 f64)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 f64)
  i32.const 0
  global.get $assembly/index/N_BODIES
  call $~lib/array/Array<assembly/index/Point>#constructor
  local.set $0
  f64.const 0
  local.set $1
  f64.const 0
  local.set $2
  f64.const 5
  local.set $3
  i32.const 0
  local.set $4
  loop $for-loop|0
   local.get $4
   i32.const 1000
   i32.lt_s
   local.set $5
   local.get $5
   if
    local.get $0
    local.get $4
    i32.const 0
    local.get $1
    local.get $2
    f64.const 0
    f64.const 0
    local.get $3
    call $assembly/index/Point#constructor
    local.tee $6
    call $~lib/array/Array<assembly/index/Point>#__unchecked_set
    local.get $4
    i32.const 10
    i32.rem_s
    i32.const 0
    i32.eq
    if
     local.get $2
     f64.const 10
     f64.add
     local.set $2
     f64.const 0
     local.set $1
    end
    local.get $1
    f64.const 1
    f64.add
    local.set $1
    local.get $6
    call $~lib/rt/stub/__release
    local.get $4
    i32.const 1
    i32.add
    local.set $4
    br $for-loop|0
   end
  end
  i32.const 0
  local.get $0
  call $assembly/index/System#constructor
  local.set $6
  global.get $assembly/index/system
  call $~lib/rt/stub/__release
  local.get $6
  global.set $assembly/index/system
  global.get $assembly/index/system
  call $assembly/index/System#energy
  local.set $7
  local.get $0
  call $~lib/rt/stub/__release
  local.get $7
 )
 (func $assembly/index/System#advance (; 17 ;) (param $0 i32) (param $1 f64) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 f64)
  (local $9 f64)
  (local $10 f64)
  (local $11 f64)
  (local $12 f64)
  (local $13 i32)
  (local $14 i32)
  (local $15 i32)
  (local $16 i32)
  (local $17 f64)
  (local $18 f64)
  (local $19 f64)
  (local $20 f64)
  (local $21 f64)
  (local $22 f64)
  (local $23 f64)
  local.get $0
  i32.load
  call $~lib/rt/stub/__retain
  local.set $2
  local.get $2
  call $~lib/array/Array<assembly/index/Point>#get:length
  local.set $3
  i32.const 0
  local.set $4
  loop $for-loop|0
   local.get $4
   local.get $3
   i32.lt_s
   local.set $5
   local.get $5
   if
    local.get $2
    local.get $4
    call $~lib/array/Array<assembly/index/Point>#__unchecked_get
    local.tee $6
    call $~lib/rt/stub/__retain
    local.set $7
    local.get $7
    f64.load
    local.set $8
    local.get $7
    f64.load offset=8
    local.set $9
    local.get $7
    f64.load offset=16
    local.set $10
    local.get $7
    f64.load offset=24
    local.set $11
    local.get $7
    f64.load offset=32
    local.set $12
    local.get $4
    i32.const 1
    i32.add
    local.set $13
    loop $for-loop|1
     local.get $13
     local.get $3
     i32.lt_s
     local.set $14
     local.get $14
     if
      local.get $2
      local.get $13
      call $~lib/array/Array<assembly/index/Point>#__unchecked_get
      local.tee $15
      call $~lib/rt/stub/__retain
      local.set $16
      local.get $8
      local.get $16
      f64.load
      f64.sub
      local.set $17
      local.get $9
      local.get $16
      f64.load offset=8
      f64.sub
      local.set $18
      local.get $17
      local.get $17
      f64.mul
      local.get $18
      local.get $18
      f64.mul
      f64.add
      local.set $19
      local.get $19
      local.set $20
      local.get $20
      f64.sqrt
      local.set $20
      local.get $1
      local.get $19
      local.get $20
      f64.mul
      f64.div
      local.set $21
      local.get $12
      local.get $21
      f64.mul
      local.set $22
      local.get $16
      f64.load offset=32
      local.get $21
      f64.mul
      local.set $23
      local.get $10
      local.get $17
      local.get $23
      f64.mul
      f64.sub
      local.set $10
      local.get $11
      local.get $18
      local.get $23
      f64.mul
      f64.sub
      local.set $11
      local.get $16
      local.get $16
      f64.load offset=16
      local.get $17
      local.get $22
      f64.mul
      f64.add
      f64.store offset=16
      local.get $16
      local.get $16
      f64.load offset=24
      local.get $18
      local.get $22
      f64.mul
      f64.add
      f64.store offset=24
      local.get $15
      call $~lib/rt/stub/__release
      local.get $16
      call $~lib/rt/stub/__release
      local.get $13
      i32.const 1
      i32.add
      local.set $13
      br $for-loop|1
     end
    end
    local.get $7
    local.get $10
    f64.store offset=16
    local.get $7
    local.get $11
    f64.store offset=24
    local.get $7
    local.get $7
    f64.load
    local.get $1
    local.get $10
    f64.mul
    f64.add
    f64.store
    local.get $7
    local.get $7
    f64.load offset=8
    local.get $1
    local.get $11
    f64.mul
    f64.add
    f64.store offset=8
    local.get $6
    call $~lib/rt/stub/__release
    local.get $7
    call $~lib/rt/stub/__release
    local.get $4
    i32.const 1
    i32.add
    local.set $4
    br $for-loop|0
   end
  end
  local.get $0
  call $~lib/rt/stub/__retain
  local.set $4
  local.get $2
  call $~lib/rt/stub/__release
  local.get $4
 )
 (func $assembly/index/step (; 18 ;) (result f64)
  global.get $assembly/index/system
  f64.const 0.1
  call $assembly/index/System#advance
  call $~lib/rt/stub/__release
  global.get $assembly/index/system
  call $assembly/index/System#energy
 )
 (func $assembly/index/e (; 19 ;) (result f64)
  global.get $assembly/index/system
  call $assembly/index/System#energy
 )
 (func $assembly/index/bench (; 20 ;) (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  i32.const 0
  local.set $1
  loop $for-loop|0
   local.get $1
   local.get $0
   i32.lt_s
   local.set $2
   local.get $2
   if
    global.get $assembly/index/system
    f64.const 0.01
    call $assembly/index/System#advance
    call $~lib/rt/stub/__release
    local.get $1
    i32.const 1
    i32.add
    local.set $1
    br $for-loop|0
   end
  end
 )
 (func $assembly/index/getBody (; 21 ;) (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  global.get $assembly/index/system
  i32.load
  call $~lib/rt/stub/__retain
  local.set $1
  local.get $0
  local.get $1
  call $~lib/array/Array<assembly/index/Point>#get:length
  i32.lt_u
  if (result i32)
   local.get $1
   local.get $0
   call $~lib/array/Array<assembly/index/Point>#__unchecked_get
   local.tee $2
  else
   i32.const 0
   call $~lib/rt/stub/__retain
  end
  local.set $2
  local.get $1
  call $~lib/rt/stub/__release
  local.get $2
 )
 (func $~start (; 22 ;)
  global.get $~lib/heap/__heap_base
  i32.const 15
  i32.add
  i32.const 15
  i32.const -1
  i32.xor
  i32.and
  global.set $~lib/rt/stub/startOffset
  global.get $~lib/rt/stub/startOffset
  global.set $~lib/rt/stub/offset
 )
)
