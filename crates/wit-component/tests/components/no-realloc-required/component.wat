(component
  (type (;0;) (func (param "s" string)))
  (type (;1;) 
    (instance
      (alias outer 1 0 (type (;0;)))
      (export "log" (func (type 0)))
    )
  )
  (import "foo" (instance (;0;) (type 1)))
  (core module (;0;)
    (type (;0;) (func (param i32 i32)))
    (import "foo" "log" (func (;0;) (type 0)))
    (memory (;0;) 1)
    (export "memory" (memory 0))
  )
  (core module (;1;)
    (type (;0;) (func (param i32 i32)))
    (func (;0;) (type 0) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 0
      call_indirect (type 0)
    )
    (table (;0;) 1 1 funcref)
    (export "0" (func 0))
    (export "$imports" (table 0))
  )
  (core module (;2;)
    (type (;0;) (func (param i32 i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "$imports" (table (;0;) 1 1 funcref))
    (elem (;0;) (i32.const 0) func 0)
  )
  (core instance (;0;) (instantiate 1))
  (alias core export 0 "0" (core func (;0;)))
  (core instance (;1;) 
    (export "log" (func 0))
  )
  (core instance (;2;) (instantiate 0
      (with "foo" (instance 1))
    )
  )
  (alias core export 2 "memory" (core memory (;0;)))
  (alias core export 0 "$imports" (core table (;0;)))
  (alias export 0 "log" (func (;0;)))
  (core func (;1;) (canon lower (func 0) (memory 0) string-encoding=utf8))
  (core instance (;3;) 
    (export "$imports" (table 0))
    (export "0" (func 1))
  )
  (core instance (;4;) (instantiate 2
      (with "" (instance 3))
    )
  )
)