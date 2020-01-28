(module
 (type $i32_=>_none (func (param i32)))
 (type $none_=>_none (func))
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $none_=>_f64 (func (result f64)))
 (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
 (type $f64_f64_=>_i32 (func (param f64 f64) (result i32)))
 (type $i32_=>_f64 (func (param i32) (result f64)))
 (import "env" "memory" (memory $0 1))
 (data (i32.const 16) "\1c\00\00\00\01\00\00\00\01\00\00\00\1c\00\00\00I\00n\00v\00a\00l\00i\00d\00 \00l\00e\00n\00g\00t\00h")
 (data (i32.const 64) "&\00\00\00\01\00\00\00\01\00\00\00&\00\00\00~\00l\00i\00b\00/\00a\00r\00r\00a\00y\00b\00u\00f\00f\00e\00r\00.\00t\00s")
 (data (i32.const 128) "\06\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\93 \00\00\02")
 (global $~lib/rt/stub/startOffset (mut i32) (i32.const 0))
 (global $~lib/rt/stub/offset (mut i32) (i32.const 0))
 (global $assembly/index/system (mut i32) (i32.const 0))
 (global $~lib/rt/__rtti_base i32 (i32.const 128))
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
 (func $~lib/rt/stub/maybeGrowMemory (; 0 ;) (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  local.get $0
  memory.size
  local.tee $2
  i32.const 16
  i32.shl
  local.tee $1
  i32.gt_u
  if
   local.get $2
   local.get $0
   local.get $1
   i32.sub
   i32.const 65535
   i32.add
   i32.const -65536
   i32.and
   i32.const 16
   i32.shr_u
   local.tee $1
   local.get $2
   local.get $1
   i32.gt_s
   select
   memory.grow
   i32.const 0
   i32.lt_s
   if
    local.get $1
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
 (func $~lib/rt/stub/__alloc (; 1 ;) (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  local.get $0
  i32.const 1073741808
  i32.gt_u
  if
   unreachable
  end
  global.get $~lib/rt/stub/offset
  i32.const 16
  i32.add
  local.tee $3
  local.get $0
  i32.const 15
  i32.add
  i32.const -16
  i32.and
  local.tee $2
  i32.const 16
  local.get $2
  i32.const 16
  i32.gt_u
  select
  local.tee $4
  i32.add
  call $~lib/rt/stub/maybeGrowMemory
  local.get $3
  i32.const 16
  i32.sub
  local.tee $2
  local.get $4
  i32.store
  local.get $2
  i32.const 1
  i32.store offset=4
  local.get $2
  local.get $1
  i32.store offset=8
  local.get $2
  local.get $0
  i32.store offset=12
  local.get $3
 )
 (func $~lib/rt/stub/__retain (; 2 ;) (param $0 i32) (result i32)
  local.get $0
 )
 (func $~lib/rt/stub/__release (; 3 ;) (param $0 i32)
  nop
 )
 (func $~lib/rt/stub/__collect (; 4 ;)
  nop
 )
 (func $~lib/rt/stub/__reset (; 5 ;)
  global.get $~lib/rt/stub/startOffset
  global.set $~lib/rt/stub/offset
 )
 (func $~lib/memory/memory.fill (; 6 ;) (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  block $~lib/util/memory/memset|inlined.0
   local.get $0
   i32.const 0
   i32.store8
   local.get $0
   i32.const 4000
   i32.add
   local.tee $1
   i32.const 1
   i32.sub
   i32.const 0
   i32.store8
   local.get $0
   i32.const 1
   i32.add
   i32.const 0
   i32.store8
   local.get $0
   i32.const 2
   i32.add
   i32.const 0
   i32.store8
   local.get $1
   i32.const 2
   i32.sub
   i32.const 0
   i32.store8
   local.get $1
   i32.const 3
   i32.sub
   i32.const 0
   i32.store8
   local.get $0
   i32.const 3
   i32.add
   i32.const 0
   i32.store8
   local.get $1
   i32.const 4
   i32.sub
   i32.const 0
   i32.store8
   i32.const 4000
   i32.const 0
   local.get $0
   i32.sub
   i32.const 3
   i32.and
   local.tee $1
   i32.sub
   local.set $2
   local.get $0
   local.get $1
   i32.add
   local.tee $0
   i32.const 0
   i32.store
   local.get $0
   local.get $2
   i32.const -4
   i32.and
   local.tee $1
   i32.add
   i32.const 4
   i32.sub
   i32.const 0
   i32.store
   local.get $1
   i32.const 8
   i32.le_u
   br_if $~lib/util/memory/memset|inlined.0
   local.get $0
   i32.const 4
   i32.add
   i32.const 0
   i32.store
   local.get $0
   i32.const 8
   i32.add
   i32.const 0
   i32.store
   local.get $0
   local.get $1
   i32.add
   local.tee $2
   i32.const 12
   i32.sub
   i32.const 0
   i32.store
   local.get $2
   i32.const 8
   i32.sub
   i32.const 0
   i32.store
   local.get $1
   i32.const 24
   i32.le_u
   br_if $~lib/util/memory/memset|inlined.0
   local.get $0
   i32.const 12
   i32.add
   i32.const 0
   i32.store
   local.get $0
   i32.const 16
   i32.add
   i32.const 0
   i32.store
   local.get $0
   i32.const 20
   i32.add
   i32.const 0
   i32.store
   local.get $0
   i32.const 24
   i32.add
   i32.const 0
   i32.store
   local.get $0
   local.get $1
   i32.add
   local.tee $2
   i32.const 28
   i32.sub
   i32.const 0
   i32.store
   local.get $2
   i32.const 24
   i32.sub
   i32.const 0
   i32.store
   local.get $2
   i32.const 20
   i32.sub
   i32.const 0
   i32.store
   local.get $2
   i32.const 16
   i32.sub
   i32.const 0
   i32.store
   local.get $0
   local.get $0
   i32.const 4
   i32.and
   i32.const 24
   i32.add
   local.tee $2
   i32.add
   local.set $0
   local.get $1
   local.get $2
   i32.sub
   local.set $1
   loop $while-continue|0
    local.get $1
    i32.const 32
    i32.ge_u
    if
     local.get $0
     i64.const 0
     i64.store
     local.get $0
     i32.const 8
     i32.add
     i64.const 0
     i64.store
     local.get $0
     i32.const 16
     i32.add
     i64.const 0
     i64.store
     local.get $0
     i32.const 24
     i32.add
     i64.const 0
     i64.store
     local.get $1
     i32.const 32
     i32.sub
     local.set $1
     local.get $0
     i32.const 32
     i32.add
     local.set $0
     br $while-continue|0
    end
   end
  end
 )
 (func $~lib/arraybuffer/ArrayBufferView#constructor (; 7 ;) (param $0 i32) (result i32)
  (local $1 i32)
  i32.const 4000
  i32.const 0
  call $~lib/rt/stub/__alloc
  local.tee $1
  call $~lib/memory/memory.fill
  local.get $0
  i32.eqz
  if
   i32.const 12
   i32.const 2
   call $~lib/rt/stub/__alloc
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
  i32.load
  drop
  local.get $0
  local.get $1
  i32.store
  local.get $0
  local.get $1
  i32.store offset=4
  local.get $0
  i32.const 4000
  i32.store offset=8
  local.get $0
 )
 (func $assembly/index/Point#constructor (; 8 ;) (param $0 f64) (param $1 f64) (result i32)
  (local $2 i32)
  i32.const 40
  i32.const 4
  call $~lib/rt/stub/__alloc
  local.tee $2
  local.get $0
  f64.store
  local.get $2
  local.get $1
  f64.store offset=8
  local.get $2
  f64.const 0
  f64.store offset=16
  local.get $2
  f64.const 0
  f64.store offset=24
  local.get $2
  f64.const 5
  f64.store offset=32
  local.get $2
 )
 (func $assembly/index/System#energy (; 9 ;) (param $0 i32) (result f64)
  (local $1 f64)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 f64)
  (local $7 f64)
  (local $8 f64)
  (local $9 f64)
  local.get $0
  i32.load
  local.tee $3
  i32.load offset=12
  local.set $4
  loop $for-loop|0
   local.get $2
   local.get $4
   i32.lt_s
   if
    local.get $3
    i32.load offset=4
    local.get $2
    i32.const 2
    i32.shl
    i32.add
    i32.load
    local.tee $0
    f64.load
    local.set $7
    local.get $0
    f64.load offset=8
    local.set $8
    local.get $1
    f64.const 0.5
    local.get $0
    f64.load offset=32
    local.tee $9
    f64.mul
    local.get $0
    f64.load offset=16
    local.tee $1
    local.get $1
    f64.mul
    local.get $0
    f64.load offset=24
    local.tee $1
    local.get $1
    f64.mul
    f64.add
    f64.mul
    f64.add
    local.set $1
    local.get $2
    i32.const 1
    i32.add
    local.set $0
    loop $for-loop|1
     local.get $0
     local.get $4
     i32.lt_s
     if
      local.get $7
      local.get $3
      i32.load offset=4
      local.get $0
      i32.const 2
      i32.shl
      i32.add
      i32.load
      local.tee $5
      f64.load
      f64.sub
      local.set $6
      local.get $1
      local.get $9
      local.get $5
      f64.load offset=32
      f64.mul
      local.get $6
      local.get $6
      f64.mul
      local.get $8
      local.get $5
      f64.load offset=8
      f64.sub
      local.tee $1
      local.get $1
      f64.mul
      f64.add
      f64.sqrt
      f64.div
      f64.sub
      local.set $1
      local.get $0
      i32.const 1
      i32.add
      local.set $0
      br $for-loop|1
     end
    end
    local.get $2
    i32.const 1
    i32.add
    local.set $2
    br $for-loop|0
   end
  end
  local.get $1
 )
 (func $assembly/index/init (; 10 ;) (result f64)
  (local $0 i32)
  (local $1 i32)
  (local $2 f64)
  (local $3 f64)
  (local $4 i32)
  (local $5 i32)
  i32.const 16
  i32.const 5
  call $~lib/rt/stub/__alloc
  call $~lib/arraybuffer/ArrayBufferView#constructor
  local.tee $1
  i32.const 0
  i32.store offset=12
  local.get $1
  i32.const 1000
  i32.store offset=12
  loop $for-loop|0
   local.get $0
   i32.const 1000
   i32.lt_s
   if
    local.get $2
    local.get $3
    call $assembly/index/Point#constructor
    local.tee $4
    local.get $1
    i32.load offset=4
    local.get $0
    i32.const 2
    i32.shl
    i32.add
    local.tee $5
    i32.load
    i32.ne
    if
     local.get $5
     local.get $4
     i32.store
    end
    local.get $0
    i32.const 10
    i32.rem_s
    if (result f64)
     local.get $2
    else
     local.get $3
     f64.const 10
     f64.add
     local.set $3
     f64.const 0
    end
    f64.const 1
    f64.add
    local.set $2
    local.get $0
    i32.const 1
    i32.add
    local.set $0
    br $for-loop|0
   end
  end
  i32.const 4
  i32.const 3
  call $~lib/rt/stub/__alloc
  local.tee $0
  local.get $1
  i32.store
  local.get $0
  global.set $assembly/index/system
  global.get $assembly/index/system
  call $assembly/index/System#energy
 )
 (func $assembly/index/step (; 11 ;) (result f64)
  (local $0 i32)
  (local $1 i32)
  (local $2 f64)
  (local $3 i32)
  (local $4 f64)
  (local $5 f64)
  (local $6 i32)
  (local $7 f64)
  (local $8 f64)
  (local $9 f64)
  (local $10 i32)
  (local $11 i32)
  (local $12 f64)
  (local $13 f64)
  (local $14 f64)
  global.get $assembly/index/system
  i32.load
  local.tee $10
  i32.load offset=12
  local.set $11
  loop $for-loop|0
   local.get $3
   local.get $11
   i32.lt_s
   if
    local.get $10
    i32.load offset=4
    local.get $3
    i32.const 2
    i32.shl
    i32.add
    i32.load
    local.tee $0
    f64.load
    local.set $12
    local.get $0
    f64.load offset=8
    local.set $13
    local.get $0
    f64.load offset=16
    local.set $4
    local.get $0
    f64.load offset=24
    local.set $5
    local.get $0
    f64.load offset=32
    local.set $14
    local.get $3
    i32.const 1
    i32.add
    local.set $6
    loop $for-loop|1
     local.get $6
     local.get $11
     i32.lt_s
     if
      local.get $12
      local.get $10
      i32.load offset=4
      local.get $6
      i32.const 2
      i32.shl
      i32.add
      i32.load
      local.tee $1
      f64.load
      f64.sub
      local.tee $2
      local.get $2
      f64.mul
      local.get $13
      local.get $1
      f64.load offset=8
      f64.sub
      local.tee $7
      local.get $7
      f64.mul
      f64.add
      local.tee $8
      f64.sqrt
      local.set $9
      local.get $4
      local.get $2
      local.get $1
      f64.load offset=32
      f64.const 0.1
      local.get $8
      local.get $9
      f64.mul
      f64.div
      local.tee $8
      f64.mul
      local.tee $9
      f64.mul
      f64.sub
      local.set $4
      local.get $5
      local.get $7
      local.get $9
      f64.mul
      f64.sub
      local.set $5
      local.get $1
      local.get $1
      f64.load offset=16
      local.get $2
      local.get $14
      local.get $8
      f64.mul
      local.tee $2
      f64.mul
      f64.add
      f64.store offset=16
      local.get $1
      local.get $1
      f64.load offset=24
      local.get $7
      local.get $2
      f64.mul
      f64.add
      f64.store offset=24
      local.get $6
      i32.const 1
      i32.add
      local.set $6
      br $for-loop|1
     end
    end
    local.get $0
    local.get $4
    f64.store offset=16
    local.get $0
    local.get $5
    f64.store offset=24
    local.get $0
    local.get $0
    f64.load
    f64.const 0.1
    local.get $4
    f64.mul
    f64.add
    f64.store
    local.get $0
    local.get $0
    f64.load offset=8
    f64.const 0.1
    local.get $5
    f64.mul
    f64.add
    f64.store offset=8
    local.get $3
    i32.const 1
    i32.add
    local.set $3
    br $for-loop|0
   end
  end
  global.get $assembly/index/system
  call $assembly/index/System#energy
 )
 (func $assembly/index/e (; 12 ;) (result f64)
  global.get $assembly/index/system
  call $assembly/index/System#energy
 )
 (func $assembly/index/bench (; 13 ;) (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 f64)
  (local $5 f64)
  (local $6 f64)
  (local $7 i32)
  (local $8 f64)
  (local $9 f64)
  (local $10 f64)
  (local $11 i32)
  (local $12 i32)
  (local $13 i32)
  (local $14 f64)
  (local $15 f64)
  (local $16 f64)
  loop $for-loop|0
   local.get $11
   local.get $0
   i32.lt_s
   if
    global.get $assembly/index/system
    i32.load
    local.tee $12
    i32.load offset=12
    local.set $13
    i32.const 0
    local.set $3
    loop $for-loop|1
     local.get $3
     local.get $13
     i32.lt_s
     if
      local.get $12
      i32.load offset=4
      local.get $3
      i32.const 2
      i32.shl
      i32.add
      i32.load
      local.tee $1
      f64.load
      local.set $14
      local.get $1
      f64.load offset=8
      local.set $15
      local.get $1
      f64.load offset=16
      local.set $5
      local.get $1
      f64.load offset=24
      local.set $6
      local.get $1
      f64.load offset=32
      local.set $16
      local.get $3
      i32.const 1
      i32.add
      local.set $7
      loop $for-loop|2
       local.get $7
       local.get $13
       i32.lt_s
       if
        local.get $14
        local.get $12
        i32.load offset=4
        local.get $7
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.tee $2
        f64.load
        f64.sub
        local.tee $4
        local.get $4
        f64.mul
        local.get $15
        local.get $2
        f64.load offset=8
        f64.sub
        local.tee $8
        local.get $8
        f64.mul
        f64.add
        local.tee $9
        f64.sqrt
        local.set $10
        local.get $5
        local.get $4
        local.get $2
        f64.load offset=32
        f64.const 0.01
        local.get $9
        local.get $10
        f64.mul
        f64.div
        local.tee $9
        f64.mul
        local.tee $10
        f64.mul
        f64.sub
        local.set $5
        local.get $6
        local.get $8
        local.get $10
        f64.mul
        f64.sub
        local.set $6
        local.get $2
        local.get $2
        f64.load offset=16
        local.get $4
        local.get $16
        local.get $9
        f64.mul
        local.tee $4
        f64.mul
        f64.add
        f64.store offset=16
        local.get $2
        local.get $2
        f64.load offset=24
        local.get $8
        local.get $4
        f64.mul
        f64.add
        f64.store offset=24
        local.get $7
        i32.const 1
        i32.add
        local.set $7
        br $for-loop|2
       end
      end
      local.get $1
      local.get $5
      f64.store offset=16
      local.get $1
      local.get $6
      f64.store offset=24
      local.get $1
      local.get $1
      f64.load
      f64.const 0.01
      local.get $5
      f64.mul
      f64.add
      f64.store
      local.get $1
      local.get $1
      f64.load offset=8
      f64.const 0.01
      local.get $6
      f64.mul
      f64.add
      f64.store offset=8
      local.get $3
      i32.const 1
      i32.add
      local.set $3
      br $for-loop|1
     end
    end
    local.get $11
    i32.const 1
    i32.add
    local.set $11
    br $for-loop|0
   end
  end
 )
 (func $assembly/index/getBody (; 14 ;) (param $0 i32) (result i32)
  (local $1 i32)
  local.get $0
  global.get $assembly/index/system
  i32.load
  local.tee $1
  i32.load offset=12
  i32.lt_u
  if (result i32)
   local.get $1
   i32.load offset=4
   local.get $0
   i32.const 2
   i32.shl
   i32.add
   i32.load
  else
   i32.const 0
  end
 )
 (func $~start (; 15 ;)
  i32.const 192
  global.set $~lib/rt/stub/startOffset
  i32.const 192
  global.set $~lib/rt/stub/offset
 )
)
