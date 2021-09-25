(module
(import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
(func $baz (param $a i32)(param $b i32)(result i32)
(i32.mul(local.get $a)(local.get $b)))
(func $main (result i32)(local $foo i32)(local $bar i32)
(local.set $foo(i32.add(i32.const 2)(i32.div_s(i32.mul(i32.const 3)(i32.add(i32.sub(i32.const 5)(i32.const 1))(i32.const 1)))(i32.const 3))))
(local.set $bar(i32.sub(i32.const 10)(i32.const 4)))
(call $baz (local.get $foo)(local.get $bar)))
(func $malloc (param i32) (result i32)
  local.get 0
  call $dlmalloc)
(func $dlmalloc (param i32) (result i32)
  (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
  global.get $mem
  i32.const 16
  i32.sub
  local.tee 1
  global.set $mem
  block
    block
      block
        block
          block
            block
              block
                block
                  block
                    block
                      block
                        block
                          local.get 0
                          i32.const 236
                          i32.gt_u
                          br_if 0
                          block
                            i32.const 0
                            i32.load offset=3392
                            local.tee 2
                            i32.const 16
                            local.get 0
                            i32.const 19
                            i32.add
                            i32.const -16
                            i32.and
                            local.get 0
                            i32.const 11
                            i32.lt_u
                            select
                            local.tee 3
                            i32.const 3
                            i32.shr_u
                            local.tee 4
                            i32.shr_u
                            local.tee 0
                            i32.const 3
                            i32.and
                            i32.eqz
                            br_if 0
                            local.get 0
                            i32.const 1
                            i32.and
                            local.get 4
                            i32.or
                            i32.const 1
                            i32.xor
                            local.tee 3
                            i32.const 3
                            i32.shl
                            local.tee 5
                            i32.const 3440
                            i32.add
                            i32.load
                            local.tee 4
                            i32.const 8
                            i32.add
                            local.set 0
                            block
                              block
                                local.get 4
                                i32.load offset=8
                                local.tee 6
                                local.get 5
                                i32.const 3432
                                i32.add
                                local.tee 5
                                i32.ne
                                br_if 0
                                i32.const 0
                                local.get 2
                                i32.const -2
                                local.get 3
                                i32.rotl
                                i32.and
                                i32.store offset=3392
                                br 1
                              end
                              i32.const 0
                              i32.load offset=3408
                              local.get 6
                              i32.gt_u
                              drop
                              local.get 5
                              local.get 6
                              i32.store offset=8
                              local.get 6
                              local.get 5
                              i32.store offset=12
                            end
                            local.get 4
                            local.get 3
                            i32.const 3
                            i32.shl
                            local.tee 6
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 4
                            local.get 6
                            i32.add
                            local.tee 4
                            local.get 4
                            i32.load offset=4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            br 12
                          end
                          local.get 3
                          i32.const 0
                          i32.load offset=3400
                          local.tee 7
                          i32.le_u
                          br_if 1
                          block
                            local.get 0
                            i32.eqz
                            br_if 0
                            block
                              block
                                local.get 0
                                local.get 4
                                i32.shl
                                i32.const 2
                                local.get 4
                                i32.shl
                                local.tee 0
                                i32.const 0
                                local.get 0
                                i32.sub
                                i32.or
                                i32.and
                                local.tee 0
                                i32.const 0
                                local.get 0
                                i32.sub
                                i32.and
                                i32.const -1
                                i32.add
                                local.tee 0
                                local.get 0
                                i32.const 12
                                i32.shr_u
                                i32.const 16
                                i32.and
                                local.tee 0
                                i32.shr_u
                                local.tee 4
                                i32.const 5
                                i32.shr_u
                                i32.const 8
                                i32.and
                                local.tee 6
                                local.get 0
                                i32.or
                                local.get 4
                                local.get 6
                                i32.shr_u
                                local.tee 0
                                i32.const 2
                                i32.shr_u
                                i32.const 4
                                i32.and
                                local.tee 4
                                i32.or
                                local.get 0
                                local.get 4
                                i32.shr_u
                                local.tee 0
                                i32.const 1
                                i32.shr_u
                                i32.const 2
                                i32.and
                                local.tee 4
                                i32.or
                                local.get 0
                                local.get 4
                                i32.shr_u
                                local.tee 0
                                i32.const 1
                                i32.shr_u
                                i32.const 1
                                i32.and
                                local.tee 4
                                i32.or
                                local.get 0
                                local.get 4
                                i32.shr_u
                                i32.add
                                local.tee 6
                                i32.const 3
                                i32.shl
                                local.tee 5
                                i32.const 3440
                                i32.add
                                i32.load
                                local.tee 4
                                i32.load offset=8
                                local.tee 0
                                local.get 5
                                i32.const 3432
                                i32.add
                                local.tee 5
                                i32.ne
                                br_if 0
                                i32.const 0
                                local.get 2
                                i32.const -2
                                local.get 6
                                i32.rotl
                                i32.and
                                local.tee 2
                                i32.store offset=3392
                                br 1
                              end
                              i32.const 0
                              i32.load offset=3408
                              local.get 0
                              i32.gt_u
                              drop
                              local.get 5
                              local.get 0
                              i32.store offset=8
                              local.get 0
                              local.get 5
                              i32.store offset=12
                            end
                            local.get 4
                            i32.const 8
                            i32.add
                            local.set 0
                            local.get 4
                            local.get 3
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 4
                            local.get 6
                            i32.const 3
                            i32.shl
                            local.tee 6
                            i32.add
                            local.get 6
                            local.get 3
                            i32.sub
                            local.tee 6
                            i32.store
                            local.get 4
                            local.get 3
                            i32.add
                            local.tee 5
                            local.get 6
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            block
                              local.get 7
                              i32.eqz
                              br_if 0
                              local.get 7
                              i32.const 3
                              i32.shr_u
                              local.tee 8
                              i32.const 3
                              i32.shl
                              i32.const 3432
                              i32.add
                              local.set 3
                              i32.const 0
                              i32.load offset=3412
                              local.set 4
                              block
                                block
                                  local.get 2
                                  i32.const 1
                                  local.get 8
                                  i32.shl
                                  local.tee 8
                                  i32.and
                                  br_if 0
                                  i32.const 0
                                  local.get 2
                                  local.get 8
                                  i32.or
                                  i32.store offset=3392
                                  local.get 3
                                  local.set 8
                                  br 1
                                end
                                local.get 3
                                i32.load offset=8
                                local.set 8
                              end
                              local.get 8
                              local.get 4
                              i32.store offset=12
                              local.get 3
                              local.get 4
                              i32.store offset=8
                              local.get 4
                              local.get 3
                              i32.store offset=12
                              local.get 4
                              local.get 8
                              i32.store offset=8
                            end
                            i32.const 0
                            local.get 5
                            i32.store offset=3412
                            i32.const 0
                            local.get 6
                            i32.store offset=3400
                            br 12
                          end
                          i32.const 0
                          i32.load offset=3396
                          local.tee 9
                          i32.eqz
                          br_if 1
                          local.get 9
                          i32.const 0
                          local.get 9
                          i32.sub
                          i32.and
                          i32.const -1
                          i32.add
                          local.tee 0
                          local.get 0
                          i32.const 12
                          i32.shr_u
                          i32.const 16
                          i32.and
                          local.tee 0
                          i32.shr_u
                          local.tee 4
                          i32.const 5
                          i32.shr_u
                          i32.const 8
                          i32.and
                          local.tee 6
                          local.get 0
                          i32.or
                          local.get 4
                          local.get 6
                          i32.shr_u
                          local.tee 0
                          i32.const 2
                          i32.shr_u
                          i32.const 4
                          i32.and
                          local.tee 4
                          i32.or
                          local.get 0
                          local.get 4
                          i32.shr_u
                          local.tee 0
                          i32.const 1
                          i32.shr_u
                          i32.const 2
                          i32.and
                          local.tee 4
                          i32.or
                          local.get 0
                          local.get 4
                          i32.shr_u
                          local.tee 0
                          i32.const 1
                          i32.shr_u
                          i32.const 1
                          i32.and
                          local.tee 4
                          i32.or
                          local.get 0
                          local.get 4
                          i32.shr_u
                          i32.add
                          i32.const 2
                          i32.shl
                          i32.const 3696
                          i32.add
                          i32.load
                          local.tee 5
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 3
                          i32.sub
                          local.set 4
                          local.get 5
                          local.set 6
                          block
                            loop
                              block
                                local.get 6
                                i32.load offset=16
                                local.tee 0
                                br_if 0
                                local.get 6
                                i32.const 20
                                i32.add
                                i32.load
                                local.tee 0
                                i32.eqz
                                br_if 2
                              end
                              local.get 0
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 3
                              i32.sub
                              local.tee 6
                              local.get 4
                              local.get 6
                              local.get 4
                              i32.lt_u
                              local.tee 6
                              select
                              local.set 4
                              local.get 0
                              local.get 5
                              local.get 6
                              select
                              local.set 5
                              local.get 0
                              local.set 6
                              br 0
                            end
                          end
                          local.get 5
                          i32.load offset=24
                          local.set 10
                          block
                            local.get 5
                            i32.load offset=12
                            local.tee 8
                            local.get 5
                            i32.eq
                            br_if 0
                            block
                              i32.const 0
                              i32.load offset=3408
                              local.get 5
                              i32.load offset=8
                              local.tee 0
                              i32.gt_u
                              br_if 0
                              local.get 0
                              i32.load offset=12
                              local.get 5
                              i32.ne
                              drop
                            end
                            local.get 8
                            local.get 0
                            i32.store offset=8
                            local.get 0
                            local.get 8
                            i32.store offset=12
                            br 11
                          end
                          block
                            local.get 5
                            i32.const 20
                            i32.add
                            local.tee 6
                            i32.load
                            local.tee 0
                            br_if 0
                            local.get 5
                            i32.load offset=16
                            local.tee 0
                            i32.eqz
                            br_if 3
                            local.get 5
                            i32.const 16
                            i32.add
                            local.set 6
                          end
                          loop
                            local.get 6
                            local.set 11
                            local.get 0
                            local.tee 8
                            i32.const 20
                            i32.add
                            local.tee 6
                            i32.load
                            local.tee 0
                            br_if 0
                            local.get 8
                            i32.const 16
                            i32.add
                            local.set 6
                            local.get 8
                            i32.load offset=16
                            local.tee 0
                            br_if 0
                          end
                          local.get 11
                          i32.const 0
                          i32.store
                          br 10
                        end
                        i32.const -1
                        local.set 3
                        local.get 0
                        i32.const -65
                        i32.gt_u
                        br_if 0
                        local.get 0
                        i32.const 19
                        i32.add
                        local.tee 0
                        i32.const -16
                        i32.and
                        local.set 3
                        i32.const 0
                        i32.load offset=3396
                        local.tee 7
                        i32.eqz
                        br_if 0
                        i32.const 0
                        local.set 11
                        block
                          local.get 0
                          i32.const 8
                          i32.shr_u
                          local.tee 0
                          i32.eqz
                          br_if 0
                          i32.const 31
                          local.set 11
                          local.get 3
                          i32.const 16777215
                          i32.gt_u
                          br_if 0
                          local.get 0
                          local.get 0
                          i32.const 1048320
                          i32.add
                          i32.const 16
                          i32.shr_u
                          i32.const 8
                          i32.and
                          local.tee 4
                          i32.shl
                          local.tee 0
                          local.get 0
                          i32.const 520192
                          i32.add
                          i32.const 16
                          i32.shr_u
                          i32.const 4
                          i32.and
                          local.tee 0
                          i32.shl
                          local.tee 6
                          local.get 6
                          i32.const 245760
                          i32.add
                          i32.const 16
                          i32.shr_u
                          i32.const 2
                          i32.and
                          local.tee 6
                          i32.shl
                          i32.const 15
                          i32.shr_u
                          local.get 0
                          local.get 4
                          i32.or
                          local.get 6
                          i32.or
                          i32.sub
                          local.tee 0
                          i32.const 1
                          i32.shl
                          local.get 3
                          local.get 0
                          i32.const 21
                          i32.add
                          i32.shr_u
                          i32.const 1
                          i32.and
                          i32.or
                          i32.const 28
                          i32.add
                          local.set 11
                        end
                        i32.const 0
                        local.get 3
                        i32.sub
                        local.set 6
                        block
                          block
                            block
                              block
                                local.get 11
                                i32.const 2
                                i32.shl
                                i32.const 3696
                                i32.add
                                i32.load
                                local.tee 4
                                br_if 0
                                i32.const 0
                                local.set 0
                                i32.const 0
                                local.set 8
                                br 1
                              end
                              local.get 3
                              i32.const 0
                              i32.const 25
                              local.get 11
                              i32.const 1
                              i32.shr_u
                              i32.sub
                              local.get 11
                              i32.const 31
                              i32.eq
                              select
                              i32.shl
                              local.set 5
                              i32.const 0
                              local.set 0
                              i32.const 0
                              local.set 8
                              loop
                                block
                                  local.get 4
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 3
                                  i32.sub
                                  local.tee 2
                                  local.get 6
                                  i32.ge_u
                                  br_if 0
                                  local.get 2
                                  local.set 6
                                  local.get 4
                                  local.set 8
                                  local.get 2
                                  br_if 0
                                  i32.const 0
                                  local.set 6
                                  local.get 4
                                  local.set 8
                                  local.get 4
                                  local.set 0
                                  br 3
                                end
                                local.get 0
                                local.get 4
                                i32.const 20
                                i32.add
                                i32.load
                                local.tee 2
                                local.get 2
                                local.get 4
                                local.get 5
                                i32.const 29
                                i32.shr_u
                                i32.const 4
                                i32.and
                                i32.add
                                i32.const 16
                                i32.add
                                i32.load
                                local.tee 4
                                i32.eq
                                select
                                local.get 0
                                local.get 2
                                select
                                local.set 0
                                local.get 5
                                local.get 4
                                i32.const 0
                                i32.ne
                                i32.shl
                                local.set 5
                                local.get 4
                                br_if 0
                              end
                            end
                            block
                              local.get 0
                              local.get 8
                              i32.or
                              br_if 0
                              i32.const 2
                              local.get 11
                              i32.shl
                              local.tee 0
                              i32.const 0
                              local.get 0
                              i32.sub
                              i32.or
                              local.get 7
                              i32.and
                              local.tee 0
                              i32.eqz
                              br_if 3
                              local.get 0
                              i32.const 0
                              local.get 0
                              i32.sub
                              i32.and
                              i32.const -1
                              i32.add
                              local.tee 0
                              local.get 0
                              i32.const 12
                              i32.shr_u
                              i32.const 16
                              i32.and
                              local.tee 0
                              i32.shr_u
                              local.tee 4
                              i32.const 5
                              i32.shr_u
                              i32.const 8
                              i32.and
                              local.tee 5
                              local.get 0
                              i32.or
                              local.get 4
                              local.get 5
                              i32.shr_u
                              local.tee 0
                              i32.const 2
                              i32.shr_u
                              i32.const 4
                              i32.and
                              local.tee 4
                              i32.or
                              local.get 0
                              local.get 4
                              i32.shr_u
                              local.tee 0
                              i32.const 1
                              i32.shr_u
                              i32.const 2
                              i32.and
                              local.tee 4
                              i32.or
                              local.get 0
                              local.get 4
                              i32.shr_u
                              local.tee 0
                              i32.const 1
                              i32.shr_u
                              i32.const 1
                              i32.and
                              local.tee 4
                              i32.or
                              local.get 0
                              local.get 4
                              i32.shr_u
                              i32.add
                              i32.const 2
                              i32.shl
                              i32.const 3696
                              i32.add
                              i32.load
                              local.set 0
                            end
                            local.get 0
                            i32.eqz
                            br_if 1
                          end
                          loop
                            local.get 0
                            i32.load offset=4
                            i32.const -8
                            i32.and
                            local.get 3
                            i32.sub
                            local.tee 2
                            local.get 6
                            i32.lt_u
                            local.set 5
                            block
                              local.get 0
                              i32.load offset=16
                              local.tee 4
                              br_if 0
                              local.get 0
                              i32.const 20
                              i32.add
                              i32.load
                              local.set 4
                            end
                            local.get 2
                            local.get 6
                            local.get 5
                            select
                            local.set 6
                            local.get 0
                            local.get 8
                            local.get 5
                            select
                            local.set 8
                            local.get 4
                            local.set 0
                            local.get 4
                            br_if 0
                          end
                        end
                        local.get 8
                        i32.eqz
                        br_if 0
                        local.get 6
                        i32.const 0
                        i32.load offset=3400
                        local.get 3
                        i32.sub
                        i32.ge_u
                        br_if 0
                        local.get 8
                        i32.load offset=24
                        local.set 11
                        block
                          local.get 8
                          i32.load offset=12
                          local.tee 5
                          local.get 8
                          i32.eq
                          br_if 0
                          block
                            i32.const 0
                            i32.load offset=3408
                            local.get 8
                            i32.load offset=8
                            local.tee 0
                            i32.gt_u
                            br_if 0
                            local.get 0
                            i32.load offset=12
                            local.get 8
                            i32.ne
                            drop
                          end
                          local.get 5
                          local.get 0
                          i32.store offset=8
                          local.get 0
                          local.get 5
                          i32.store offset=12
                          br 9
                        end
                        block
                          local.get 8
                          i32.const 20
                          i32.add
                          local.tee 4
                          i32.load
                          local.tee 0
                          br_if 0
                          local.get 8
                          i32.load offset=16
                          local.tee 0
                          i32.eqz
                          br_if 3
                          local.get 8
                          i32.const 16
                          i32.add
                          local.set 4
                        end
                        loop
                          local.get 4
                          local.set 2
                          local.get 0
                          local.tee 5
                          i32.const 20
                          i32.add
                          local.tee 4
                          i32.load
                          local.tee 0
                          br_if 0
                          local.get 5
                          i32.const 16
                          i32.add
                          local.set 4
                          local.get 5
                          i32.load offset=16
                          local.tee 0
                          br_if 0
                        end
                        local.get 2
                        i32.const 0
                        i32.store
                        br 8
                      end
                      block
                        i32.const 0
                        i32.load offset=3400
                        local.tee 0
                        local.get 3
                        i32.lt_u
                        br_if 0
                        i32.const 0
                        i32.load offset=3412
                        local.set 4
                        block
                          block
                            local.get 0
                            local.get 3
                            i32.sub
                            local.tee 6
                            i32.const 16
                            i32.lt_u
                            br_if 0
                            local.get 4
                            local.get 3
                            i32.add
                            local.tee 5
                            local.get 6
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            local.get 6
                            i32.store offset=3400
                            i32.const 0
                            local.get 5
                            i32.store offset=3412
                            local.get 4
                            local.get 0
                            i32.add
                            local.get 6
                            i32.store
                            local.get 4
                            local.get 3
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            br 1
                          end
                          local.get 4
                          local.get 0
                          i32.const 3
                          i32.or
                          i32.store offset=4
                          local.get 4
                          local.get 0
                          i32.add
                          local.tee 0
                          local.get 0
                          i32.load offset=4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          i32.const 0
                          i32.const 0
                          i32.store offset=3412
                          i32.const 0
                          i32.const 0
                          i32.store offset=3400
                        end
                        local.get 4
                        i32.const 8
                        i32.add
                        local.set 0
                        br 10
                      end
                      block
                        i32.const 0
                        i32.load offset=3404
                        local.tee 5
                        local.get 3
                        i32.le_u
                        br_if 0
                        i32.const 0
                        i32.load offset=3416
                        local.tee 0
                        local.get 3
                        i32.add
                        local.tee 4
                        local.get 5
                        local.get 3
                        i32.sub
                        local.tee 6
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        i32.const 0
                        local.get 6
                        i32.store offset=3404
                        i32.const 0
                        local.get 4
                        i32.store offset=3416
                        local.get 0
                        local.get 3
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 0
                        i32.const 8
                        i32.add
                        local.set 0
                        br 10
                      end
                      block
                        block
                          i32.const 0
                          i32.load offset=3864
                          i32.eqz
                          br_if 0
                          i32.const 0
                          i32.load offset=3872
                          local.set 4
                          br 1
                        end
                        i32.const 0
                        i64.const -1
                        i64.store offset=3876 align=4
                        i32.const 0
                        i64.const 281474976776192
                        i64.store offset=3868 align=4
                        i32.const 0
                        local.get 1
                        i32.const 12
                        i32.add
                        i32.const -16
                        i32.and
                        i32.const 1431655768
                        i32.xor
                        i32.store offset=3864
                        i32.const 0
                        i32.const 0
                        i32.store offset=3884
                        i32.const 0
                        i32.const 0
                        i32.store offset=3836
                        i32.const 65536
                        local.set 4
                      end
                      i32.const 0
                      local.set 0
                      block
                        local.get 4
                        local.get 3
                        i32.const 71
                        i32.add
                        local.tee 7
                        i32.add
                        local.tee 2
                        i32.const 0
                        local.get 4
                        i32.sub
                        local.tee 11
                        i32.and
                        local.tee 8
                        local.get 3
                        i32.gt_u
                        br_if 0
                        i32.const 0
                        i32.const 48
                        i32.store offset=3888
                        br 10
                      end
                      block
                        i32.const 0
                        i32.load offset=3832
                        local.tee 0
                        i32.eqz
                        br_if 0
                        block
                          i32.const 0
                          i32.load offset=3824
                          local.tee 4
                          local.get 8
                          i32.add
                          local.tee 6
                          local.get 4
                          i32.le_u
                          br_if 0
                          local.get 6
                          local.get 0
                          i32.le_u
                          br_if 1
                        end
                        i32.const 0
                        local.set 0
                        i32.const 0
                        i32.const 48
                        i32.store offset=3888
                        br 10
                      end
                      i32.const 0
                      i32.load8_u offset=3836
                      i32.const 4
                      i32.and
                      br_if 4
                      block
                        block
                          block
                            i32.const 0
                            i32.load offset=3416
                            local.tee 4
                            i32.eqz
                            br_if 0
                            i32.const 3840
                            local.set 0
                            loop
                              block
                                local.get 0
                                i32.load
                                local.tee 6
                                local.get 4
                                i32.gt_u
                                br_if 0
                                local.get 6
                                local.get 0
                                i32.load offset=4
                                i32.add
                                local.get 4
                                i32.gt_u
                                br_if 3
                              end
                              local.get 0
                              i32.load offset=8
                              local.tee 0
                              br_if 0
                            end
                          end
                          i32.const 0
                          call $sbrk
                          local.tee 5
                          i32.const -1
                          i32.eq
                          br_if 5
                          local.get 8
                          local.set 2
                          block
                            i32.const 0
                            i32.load offset=3868
                            local.tee 0
                            i32.const -1
                            i32.add
                            local.tee 4
                            local.get 5
                            i32.and
                            i32.eqz
                            br_if 0
                            local.get 8
                            local.get 5
                            i32.sub
                            local.get 4
                            local.get 5
                            i32.add
                            i32.const 0
                            local.get 0
                            i32.sub
                            i32.and
                            i32.add
                            local.set 2
                          end
                          local.get 2
                          local.get 3
                          i32.le_u
                          br_if 5
                          local.get 2
                          i32.const 2147483646
                          i32.gt_u
                          br_if 5
                          block
                            i32.const 0
                            i32.load offset=3832
                            local.tee 0
                            i32.eqz
                            br_if 0
                            i32.const 0
                            i32.load offset=3824
                            local.tee 4
                            local.get 2
                            i32.add
                            local.tee 6
                            local.get 4
                            i32.le_u
                            br_if 6
                            local.get 6
                            local.get 0
                            i32.gt_u
                            br_if 6
                          end
                          local.get 2
                          call $sbrk
                          local.tee 0
                          local.get 5
                          i32.ne
                          br_if 1
                          br 7
                        end
                        local.get 2
                        local.get 5
                        i32.sub
                        local.get 11
                        i32.and
                        local.tee 2
                        i32.const 2147483646
                        i32.gt_u
                        br_if 4
                        local.get 2
                        call $sbrk
                        local.tee 5
                        local.get 0
                        i32.load
                        local.get 0
                        i32.load offset=4
                        i32.add
                        i32.eq
                        br_if 3
                        local.get 5
                        local.set 0
                      end
                      local.get 0
                      local.set 5
                      block
                        local.get 3
                        i32.const 72
                        i32.add
                        local.get 2
                        i32.le_u
                        br_if 0
                        local.get 2
                        i32.const 2147483646
                        i32.gt_u
                        br_if 0
                        local.get 5
                        i32.const -1
                        i32.eq
                        br_if 0
                        local.get 7
                        local.get 2
                        i32.sub
                        i32.const 0
                        i32.load offset=3872
                        local.tee 0
                        i32.add
                        i32.const 0
                        local.get 0
                        i32.sub
                        i32.and
                        local.tee 0
                        i32.const 2147483646
                        i32.gt_u
                        br_if 6
                        block
                          local.get 0
                          call $sbrk
                          i32.const -1
                          i32.eq
                          br_if 0
                          local.get 0
                          local.get 2
                          i32.add
                          local.set 2
                          br 7
                        end
                        i32.const 0
                        local.get 2
                        i32.sub
                        call $sbrk
                        drop
                        br 4
                      end
                      local.get 5
                      i32.const -1
                      i32.ne
                      br_if 5
                      br 3
                    end
                    i32.const 0
                    local.set 8
                    br 7
                  end
                  i32.const 0
                  local.set 5
                  br 5
                end
                local.get 5
                i32.const -1
                i32.ne
                br_if 2
              end
              i32.const 0
              i32.const 0
              i32.load offset=3836
              i32.const 4
              i32.or
              i32.store offset=3836
            end
            local.get 8
            i32.const 2147483646
            i32.gt_u
            br_if 1
            local.get 8
            call $sbrk
            local.tee 5
            i32.const 0
            call $sbrk
            local.tee 0
            i32.ge_u
            br_if 1
            local.get 5
            i32.const -1
            i32.eq
            br_if 1
            local.get 0
            i32.const -1
            i32.eq
            br_if 1
            local.get 0
            local.get 5
            i32.sub
            local.tee 2
            local.get 3
            i32.const 56
            i32.add
            i32.le_u
            br_if 1
          end
          i32.const 0
          i32.const 0
          i32.load offset=3824
          local.get 2
          i32.add
          local.tee 0
          i32.store offset=3824
          block
            local.get 0
            i32.const 0
            i32.load offset=3828
            i32.le_u
            br_if 0
            i32.const 0
            local.get 0
            i32.store offset=3828
          end
          block
            block
              block
                block
                  i32.const 0
                  i32.load offset=3416
                  local.tee 4
                  i32.eqz
                  br_if 0
                  i32.const 3840
                  local.set 0
                  loop
                    local.get 5
                    local.get 0
                    i32.load
                    local.tee 6
                    local.get 0
                    i32.load offset=4
                    local.tee 8
                    i32.add
                    i32.eq
                    br_if 2
                    local.get 0
                    i32.load offset=8
                    local.tee 0
                    br_if 0
                    br 3
                  end
                end
                block
                  block
                    i32.const 0
                    i32.load offset=3408
                    local.tee 0
                    i32.eqz
                    br_if 0
                    local.get 5
                    local.get 0
                    i32.ge_u
                    br_if 1
                  end
                  i32.const 0
                  local.get 5
                  i32.store offset=3408
                end
                i32.const 0
                local.set 0
                i32.const 0
                local.get 2
                i32.store offset=3844
                i32.const 0
                local.get 5
                i32.store offset=3840
                i32.const 0
                i32.const -1
                i32.store offset=3424
                i32.const 0
                i32.const 0
                i32.load offset=3864
                i32.store offset=3428
                i32.const 0
                i32.const 0
                i32.store offset=3852
                loop
                  local.get 0
                  i32.const 3440
                  i32.add
                  local.get 0
                  i32.const 3432
                  i32.add
                  local.tee 4
                  i32.store
                  local.get 0
                  i32.const 3444
                  i32.add
                  local.get 4
                  i32.store
                  local.get 0
                  i32.const 8
                  i32.add
                  local.tee 0
                  i32.const 256
                  i32.ne
                  br_if 0
                end
                local.get 5
                i32.const -8
                local.get 5
                i32.sub
                i32.const 15
                i32.and
                i32.const 0
                local.get 5
                i32.const 8
                i32.add
                i32.const 15
                i32.and
                select
                local.tee 0
                i32.add
                local.tee 4
                local.get 2
                i32.const -56
                i32.add
                local.tee 6
                local.get 0
                i32.sub
                local.tee 0
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 0
                i32.const 0
                i32.load offset=3880
                i32.store offset=3420
                i32.const 0
                local.get 0
                i32.store offset=3404
                i32.const 0
                local.get 4
                i32.store offset=3416
                local.get 5
                local.get 6
                i32.add
                i32.const 56
                i32.store offset=4
                br 2
              end
              local.get 0
              i32.load8_u offset=12
              i32.const 8
              i32.and
              br_if 0
              local.get 5
              local.get 4
              i32.le_u
              br_if 0
              local.get 6
              local.get 4
              i32.gt_u
              br_if 0
              local.get 4
              i32.const -8
              local.get 4
              i32.sub
              i32.const 15
              i32.and
              i32.const 0
              local.get 4
              i32.const 8
              i32.add
              i32.const 15
              i32.and
              select
              local.tee 6
              i32.add
              local.tee 5
              i32.const 0
              i32.load offset=3404
              local.get 2
              i32.add
              local.tee 11
              local.get 6
              i32.sub
              local.tee 6
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 8
              local.get 2
              i32.add
              i32.store offset=4
              i32.const 0
              i32.const 0
              i32.load offset=3880
              i32.store offset=3420
              i32.const 0
              local.get 6
              i32.store offset=3404
              i32.const 0
              local.get 5
              i32.store offset=3416
              local.get 4
              local.get 11
              i32.add
              i32.const 56
              i32.store offset=4
              br 1
            end
            block
              local.get 5
              i32.const 0
              i32.load offset=3408
              local.tee 8
              i32.ge_u
              br_if 0
              i32.const 0
              local.get 5
              i32.store offset=3408
              local.get 5
              local.set 8
            end
            local.get 5
            local.get 2
            i32.add
            local.set 6
            i32.const 3840
            local.set 0
            block
              block
                block
                  block
                    block
                      block
                        block
                          loop
                            local.get 0
                            i32.load
                            local.get 6
                            i32.eq
                            br_if 1
                            local.get 0
                            i32.load offset=8
                            local.tee 0
                            br_if 0
                            br 2
                          end
                        end
                        local.get 0
                        i32.load8_u offset=12
                        i32.const 8
                        i32.and
                        i32.eqz
                        br_if 1
                      end
                      i32.const 3840
                      local.set 0
                      loop
                        block
                          local.get 0
                          i32.load
                          local.tee 6
                          local.get 4
                          i32.gt_u
                          br_if 0
                          local.get 6
                          local.get 0
                          i32.load offset=4
                          i32.add
                          local.tee 6
                          local.get 4
                          i32.gt_u
                          br_if 3
                        end
                        local.get 0
                        i32.load offset=8
                        local.set 0
                        br 0
                      end
                    end
                    local.get 0
                    local.get 5
                    i32.store
                    local.get 0
                    local.get 0
                    i32.load offset=4
                    local.get 2
                    i32.add
                    i32.store offset=4
                    local.get 5
                    i32.const -8
                    local.get 5
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 5
                    i32.const 8
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    i32.add
                    local.tee 11
                    local.get 3
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 6
                    i32.const -8
                    local.get 6
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 6
                    i32.const 8
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    i32.add
                    local.tee 5
                    local.get 11
                    i32.sub
                    local.get 3
                    i32.sub
                    local.set 0
                    local.get 11
                    local.get 3
                    i32.add
                    local.set 6
                    block
                      local.get 4
                      local.get 5
                      i32.ne
                      br_if 0
                      i32.const 0
                      local.get 6
                      i32.store offset=3416
                      i32.const 0
                      i32.const 0
                      i32.load offset=3404
                      local.get 0
                      i32.add
                      local.tee 0
                      i32.store offset=3404
                      local.get 6
                      local.get 0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      br 3
                    end
                    block
                      i32.const 0
                      i32.load offset=3412
                      local.get 5
                      i32.ne
                      br_if 0
                      i32.const 0
                      local.get 6
                      i32.store offset=3412
                      i32.const 0
                      i32.const 0
                      i32.load offset=3400
                      local.get 0
                      i32.add
                      local.tee 0
                      i32.store offset=3400
                      local.get 6
                      local.get 0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 6
                      local.get 0
                      i32.add
                      local.get 0
                      i32.store
                      br 3
                    end
                    block
                      local.get 5
                      i32.load offset=4
                      local.tee 4
                      i32.const 3
                      i32.and
                      i32.const 1
                      i32.ne
                      br_if 0
                      local.get 4
                      i32.const -8
                      i32.and
                      local.set 7
                      block
                        block
                          local.get 4
                          i32.const 255
                          i32.gt_u
                          br_if 0
                          local.get 5
                          i32.load offset=12
                          local.set 3
                          block
                            local.get 5
                            i32.load offset=8
                            local.tee 2
                            local.get 4
                            i32.const 3
                            i32.shr_u
                            local.tee 9
                            i32.const 3
                            i32.shl
                            i32.const 3432
                            i32.add
                            local.tee 4
                            i32.eq
                            br_if 0
                            local.get 8
                            local.get 2
                            i32.gt_u
                            drop
                          end
                          block
                            local.get 3
                            local.get 2
                            i32.ne
                            br_if 0
                            i32.const 0
                            i32.const 0
                            i32.load offset=3392
                            i32.const -2
                            local.get 9
                            i32.rotl
                            i32.and
                            i32.store offset=3392
                            br 2
                          end
                          block
                            local.get 3
                            local.get 4
                            i32.eq
                            br_if 0
                            local.get 8
                            local.get 3
                            i32.gt_u
                            drop
                          end
                          local.get 3
                          local.get 2
                          i32.store offset=8
                          local.get 2
                          local.get 3
                          i32.store offset=12
                          br 1
                        end
                        local.get 5
                        i32.load offset=24
                        local.set 9
                        block
                          block
                            local.get 5
                            i32.load offset=12
                            local.tee 2
                            local.get 5
                            i32.eq
                            br_if 0
                            block
                              local.get 8
                              local.get 5
                              i32.load offset=8
                              local.tee 4
                              i32.gt_u
                              br_if 0
                              local.get 4
                              i32.load offset=12
                              local.get 5
                              i32.ne
                              drop
                            end
                            local.get 2
                            local.get 4
                            i32.store offset=8
                            local.get 4
                            local.get 2
                            i32.store offset=12
                            br 1
                          end
                          block
                            local.get 5
                            i32.const 20
                            i32.add
                            local.tee 4
                            i32.load
                            local.tee 3
                            br_if 0
                            local.get 5
                            i32.const 16
                            i32.add
                            local.tee 4
                            i32.load
                            local.tee 3
                            br_if 0
                            i32.const 0
                            local.set 2
                            br 1
                          end
                          loop
                            local.get 4
                            local.set 8
                            local.get 3
                            local.tee 2
                            i32.const 20
                            i32.add
                            local.tee 4
                            i32.load
                            local.tee 3
                            br_if 0
                            local.get 2
                            i32.const 16
                            i32.add
                            local.set 4
                            local.get 2
                            i32.load offset=16
                            local.tee 3
                            br_if 0
                          end
                          local.get 8
                          i32.const 0
                          i32.store
                        end
                        local.get 9
                        i32.eqz
                        br_if 0
                        block
                          block
                            local.get 5
                            i32.load offset=28
                            local.tee 3
                            i32.const 2
                            i32.shl
                            i32.const 3696
                            i32.add
                            local.tee 4
                            i32.load
                            local.get 5
                            i32.ne
                            br_if 0
                            local.get 4
                            local.get 2
                            i32.store
                            local.get 2
                            br_if 1
                            i32.const 0
                            i32.const 0
                            i32.load offset=3396
                            i32.const -2
                            local.get 3
                            i32.rotl
                            i32.and
                            i32.store offset=3396
                            br 2
                          end
                          local.get 9
                          i32.const 16
                          i32.const 20
                          local.get 9
                          i32.load offset=16
                          local.get 5
                          i32.eq
                          select
                          i32.add
                          local.get 2
                          i32.store
                          local.get 2
                          i32.eqz
                          br_if 1
                        end
                        local.get 2
                        local.get 9
                        i32.store offset=24
                        block
                          local.get 5
                          i32.load offset=16
                          local.tee 4
                          i32.eqz
                          br_if 0
                          local.get 2
                          local.get 4
                          i32.store offset=16
                          local.get 4
                          local.get 2
                          i32.store offset=24
                        end
                        local.get 5
                        i32.load offset=20
                        local.tee 4
                        i32.eqz
                        br_if 0
                        local.get 2
                        i32.const 20
                        i32.add
                        local.get 4
                        i32.store
                        local.get 4
                        local.get 2
                        i32.store offset=24
                      end
                      local.get 7
                      local.get 0
                      i32.add
                      local.set 0
                      local.get 5
                      local.get 7
                      i32.add
                      local.set 5
                    end
                    local.get 5
                    local.get 5
                    i32.load offset=4
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 6
                    local.get 0
                    i32.add
                    local.get 0
                    i32.store
                    local.get 6
                    local.get 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    block
                      local.get 0
                      i32.const 255
                      i32.gt_u
                      br_if 0
                      local.get 0
                      i32.const 3
                      i32.shr_u
                      local.tee 4
                      i32.const 3
                      i32.shl
                      i32.const 3432
                      i32.add
                      local.set 0
                      block
                        block
                          i32.const 0
                          i32.load offset=3392
                          local.tee 3
                          i32.const 1
                          local.get 4
                          i32.shl
                          local.tee 4
                          i32.and
                          br_if 0
                          i32.const 0
                          local.get 3
                          local.get 4
                          i32.or
                          i32.store offset=3392
                          local.get 0
                          local.set 4
                          br 1
                        end
                        local.get 0
                        i32.load offset=8
                        local.set 4
                      end
                      local.get 4
                      local.get 6
                      i32.store offset=12
                      local.get 0
                      local.get 6
                      i32.store offset=8
                      local.get 6
                      local.get 0
                      i32.store offset=12
                      local.get 6
                      local.get 4
                      i32.store offset=8
                      br 3
                    end
                    i32.const 0
                    local.set 4
                    block
                      local.get 0
                      i32.const 8
                      i32.shr_u
                      local.tee 3
                      i32.eqz
                      br_if 0
                      i32.const 31
                      local.set 4
                      local.get 0
                      i32.const 16777215
                      i32.gt_u
                      br_if 0
                      local.get 3
                      local.get 3
                      i32.const 1048320
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 8
                      i32.and
                      local.tee 4
                      i32.shl
                      local.tee 3
                      local.get 3
                      i32.const 520192
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 4
                      i32.and
                      local.tee 3
                      i32.shl
                      local.tee 5
                      local.get 5
                      i32.const 245760
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 2
                      i32.and
                      local.tee 5
                      i32.shl
                      i32.const 15
                      i32.shr_u
                      local.get 3
                      local.get 4
                      i32.or
                      local.get 5
                      i32.or
                      i32.sub
                      local.tee 4
                      i32.const 1
                      i32.shl
                      local.get 0
                      local.get 4
                      i32.const 21
                      i32.add
                      i32.shr_u
                      i32.const 1
                      i32.and
                      i32.or
                      i32.const 28
                      i32.add
                      local.set 4
                    end
                    local.get 6
                    local.get 4
                    i32.store offset=28
                    local.get 6
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get 4
                    i32.const 2
                    i32.shl
                    i32.const 3696
                    i32.add
                    local.set 3
                    block
                      i32.const 0
                      i32.load offset=3396
                      local.tee 5
                      i32.const 1
                      local.get 4
                      i32.shl
                      local.tee 8
                      i32.and
                      br_if 0
                      local.get 3
                      local.get 6
                      i32.store
                      i32.const 0
                      local.get 5
                      local.get 8
                      i32.or
                      i32.store offset=3396
                      local.get 6
                      local.get 3
                      i32.store offset=24
                      local.get 6
                      local.get 6
                      i32.store offset=8
                      local.get 6
                      local.get 6
                      i32.store offset=12
                      br 3
                    end
                    local.get 0
                    i32.const 0
                    i32.const 25
                    local.get 4
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    local.get 4
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 4
                    local.get 3
                    i32.load
                    local.set 5
                    loop
                      local.get 5
                      local.tee 3
                      i32.load offset=4
                      i32.const -8
                      i32.and
                      local.get 0
                      i32.eq
                      br_if 2
                      local.get 4
                      i32.const 29
                      i32.shr_u
                      local.set 5
                      local.get 4
                      i32.const 1
                      i32.shl
                      local.set 4
                      local.get 3
                      local.get 5
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 8
                      i32.load
                      local.tee 5
                      br_if 0
                    end
                    local.get 8
                    local.get 6
                    i32.store
                    local.get 6
                    local.get 3
                    i32.store offset=24
                    local.get 6
                    local.get 6
                    i32.store offset=12
                    local.get 6
                    local.get 6
                    i32.store offset=8
                    br 2
                  end
                  local.get 5
                  i32.const -8
                  local.get 5
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.const 0
                  local.get 5
                  i32.const 8
                  i32.add
                  i32.const 15
                  i32.and
                  select
                  local.tee 0
                  i32.add
                  local.tee 11
                  local.get 2
                  i32.const -56
                  i32.add
                  local.tee 8
                  local.get 0
                  i32.sub
                  local.tee 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 5
                  local.get 8
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  local.get 4
                  local.get 6
                  i32.const 55
                  local.get 6
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.const 0
                  local.get 6
                  i32.const -55
                  i32.add
                  i32.const 15
                  i32.and
                  select
                  i32.add
                  i32.const -63
                  i32.add
                  local.tee 8
                  local.get 8
                  local.get 4
                  i32.const 16
                  i32.add
                  i32.lt_u
                  select
                  local.tee 8
                  i32.const 35
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=3880
                  i32.store offset=3420
                  i32.const 0
                  local.get 0
                  i32.store offset=3404
                  i32.const 0
                  local.get 11
                  i32.store offset=3416
                  local.get 8
                  i32.const 16
                  i32.add
                  i32.const 0
                  i64.load offset=3848 align=4
                  i64.store align=4
                  local.get 8
                  i32.const 0
                  i64.load offset=3840 align=4
                  i64.store offset=8 align=4
                  i32.const 0
                  local.get 8
                  i32.const 8
                  i32.add
                  i32.store offset=3848
                  i32.const 0
                  local.get 2
                  i32.store offset=3844
                  i32.const 0
                  local.get 5
                  i32.store offset=3840
                  i32.const 0
                  i32.const 0
                  i32.store offset=3852
                  local.get 8
                  i32.const 36
                  i32.add
                  local.set 0
                  loop
                    local.get 0
                    i32.const 7
                    i32.store
                    local.get 0
                    i32.const 4
                    i32.add
                    local.tee 0
                    local.get 6
                    i32.lt_u
                    br_if 0
                  end
                  local.get 8
                  local.get 4
                  i32.eq
                  br_if 3
                  local.get 8
                  local.get 8
                  i32.load offset=4
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  local.get 8
                  local.get 8
                  local.get 4
                  i32.sub
                  local.tee 2
                  i32.store
                  local.get 4
                  local.get 2
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  block
                    local.get 2
                    i32.const 255
                    i32.gt_u
                    br_if 0
                    local.get 2
                    i32.const 3
                    i32.shr_u
                    local.tee 6
                    i32.const 3
                    i32.shl
                    i32.const 3432
                    i32.add
                    local.set 0
                    block
                      block
                        i32.const 0
                        i32.load offset=3392
                        local.tee 5
                        i32.const 1
                        local.get 6
                        i32.shl
                        local.tee 6
                        i32.and
                        br_if 0
                        i32.const 0
                        local.get 5
                        local.get 6
                        i32.or
                        i32.store offset=3392
                        local.get 0
                        local.set 6
                        br 1
                      end
                      local.get 0
                      i32.load offset=8
                      local.set 6
                    end
                    local.get 6
                    local.get 4
                    i32.store offset=12
                    local.get 0
                    local.get 4
                    i32.store offset=8
                    local.get 4
                    local.get 0
                    i32.store offset=12
                    local.get 4
                    local.get 6
                    i32.store offset=8
                    br 4
                  end
                  i32.const 0
                  local.set 0
                  block
                    local.get 2
                    i32.const 8
                    i32.shr_u
                    local.tee 6
                    i32.eqz
                    br_if 0
                    i32.const 31
                    local.set 0
                    local.get 2
                    i32.const 16777215
                    i32.gt_u
                    br_if 0
                    local.get 6
                    local.get 6
                    i32.const 1048320
                    i32.add
                    i32.const 16
                    i32.shr_u
                    i32.const 8
                    i32.and
                    local.tee 0
                    i32.shl
                    local.tee 6
                    local.get 6
                    i32.const 520192
                    i32.add
                    i32.const 16
                    i32.shr_u
                    i32.const 4
                    i32.and
                    local.tee 6
                    i32.shl
                    local.tee 5
                    local.get 5
                    i32.const 245760
                    i32.add
                    i32.const 16
                    i32.shr_u
                    i32.const 2
                    i32.and
                    local.tee 5
                    i32.shl
                    i32.const 15
                    i32.shr_u
                    local.get 6
                    local.get 0
                    i32.or
                    local.get 5
                    i32.or
                    i32.sub
                    local.tee 0
                    i32.const 1
                    i32.shl
                    local.get 2
                    local.get 0
                    i32.const 21
                    i32.add
                    i32.shr_u
                    i32.const 1
                    i32.and
                    i32.or
                    i32.const 28
                    i32.add
                    local.set 0
                  end
                  local.get 4
                  i64.const 0
                  i64.store offset=16 align=4
                  local.get 4
                  i32.const 28
                  i32.add
                  local.get 0
                  i32.store
                  local.get 0
                  i32.const 2
                  i32.shl
                  i32.const 3696
                  i32.add
                  local.set 6
                  block
                    i32.const 0
                    i32.load offset=3396
                    local.tee 5
                    i32.const 1
                    local.get 0
                    i32.shl
                    local.tee 8
                    i32.and
                    br_if 0
                    local.get 6
                    local.get 4
                    i32.store
                    i32.const 0
                    local.get 5
                    local.get 8
                    i32.or
                    i32.store offset=3396
                    local.get 4
                    i32.const 24
                    i32.add
                    local.get 6
                    i32.store
                    local.get 4
                    local.get 4
                    i32.store offset=8
                    local.get 4
                    local.get 4
                    i32.store offset=12
                    br 4
                  end
                  local.get 2
                  i32.const 0
                  i32.const 25
                  local.get 0
                  i32.const 1
                  i32.shr_u
                  i32.sub
                  local.get 0
                  i32.const 31
                  i32.eq
                  select
                  i32.shl
                  local.set 0
                  local.get 6
                  i32.load
                  local.set 5
                  loop
                    local.get 5
                    local.tee 6
                    i32.load offset=4
                    i32.const -8
                    i32.and
                    local.get 2
                    i32.eq
                    br_if 3
                    local.get 0
                    i32.const 29
                    i32.shr_u
                    local.set 5
                    local.get 0
                    i32.const 1
                    i32.shl
                    local.set 0
                    local.get 6
                    local.get 5
                    i32.const 4
                    i32.and
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 8
                    i32.load
                    local.tee 5
                    br_if 0
                  end
                  local.get 8
                  local.get 4
                  i32.store
                  local.get 4
                  i32.const 24
                  i32.add
                  local.get 6
                  i32.store
                  local.get 4
                  local.get 4
                  i32.store offset=12
                  local.get 4
                  local.get 4
                  i32.store offset=8
                  br 3
                end
                local.get 3
                i32.load offset=8
                local.set 0
                local.get 3
                local.get 6
                i32.store offset=8
                local.get 0
                local.get 6
                i32.store offset=12
                local.get 6
                i32.const 0
                i32.store offset=24
                local.get 6
                local.get 0
                i32.store offset=8
                local.get 6
                local.get 3
                i32.store offset=12
              end
              local.get 11
              i32.const 8
              i32.add
              local.set 0
              br 5
            end
            local.get 6
            i32.load offset=8
            local.set 0
            local.get 6
            local.get 4
            i32.store offset=8
            local.get 0
            local.get 4
            i32.store offset=12
            local.get 4
            i32.const 24
            i32.add
            i32.const 0
            i32.store
            local.get 4
            local.get 0
            i32.store offset=8
            local.get 4
            local.get 6
            i32.store offset=12
          end
          i32.const 0
          i32.load offset=3404
          local.tee 0
          local.get 3
          i32.le_u
          br_if 0
          i32.const 0
          i32.load offset=3416
          local.tee 4
          local.get 3
          i32.add
          local.tee 6
          local.get 0
          local.get 3
          i32.sub
          local.tee 0
          i32.const 1
          i32.or
          i32.store offset=4
          i32.const 0
          local.get 0
          i32.store offset=3404
          i32.const 0
          local.get 6
          i32.store offset=3416
          local.get 4
          local.get 3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 4
          i32.const 8
          i32.add
          local.set 0
          br 3
        end
        i32.const 0
        local.set 0
        i32.const 0
        i32.const 48
        i32.store offset=3888
        br 2
      end
      block
        local.get 11
        i32.eqz
        br_if 0
        block
          block
            local.get 8
            local.get 8
            i32.load offset=28
            local.tee 4
            i32.const 2
            i32.shl
            i32.const 3696
            i32.add
            local.tee 0
            i32.load
            i32.ne
            br_if 0
            local.get 0
            local.get 5
            i32.store
            local.get 5
            br_if 1
            i32.const 0
            local.get 7
            i32.const -2
            local.get 4
            i32.rotl
            i32.and
            local.tee 7
            i32.store offset=3396
            br 2
          end
          local.get 11
          i32.const 16
          i32.const 20
          local.get 11
          i32.load offset=16
          local.get 8
          i32.eq
          select
          i32.add
          local.get 5
          i32.store
          local.get 5
          i32.eqz
          br_if 1
        end
        local.get 5
        local.get 11
        i32.store offset=24
        block
          local.get 8
          i32.load offset=16
          local.tee 0
          i32.eqz
          br_if 0
          local.get 5
          local.get 0
          i32.store offset=16
          local.get 0
          local.get 5
          i32.store offset=24
        end
        local.get 8
        i32.const 20
        i32.add
        i32.load
        local.tee 0
        i32.eqz
        br_if 0
        local.get 5
        i32.const 20
        i32.add
        local.get 0
        i32.store
        local.get 0
        local.get 5
        i32.store offset=24
      end
      block
        block
          local.get 6
          i32.const 15
          i32.gt_u
          br_if 0
          local.get 8
          local.get 6
          local.get 3
          i32.add
          local.tee 0
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 8
          local.get 0
          i32.add
          local.tee 0
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          br 1
        end
        local.get 8
        local.get 3
        i32.add
        local.tee 5
        local.get 6
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 8
        local.get 3
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 5
        local.get 6
        i32.add
        local.get 6
        i32.store
        block
          local.get 6
          i32.const 255
          i32.gt_u
          br_if 0
          local.get 6
          i32.const 3
          i32.shr_u
          local.tee 4
          i32.const 3
          i32.shl
          i32.const 3432
          i32.add
          local.set 0
          block
            block
              i32.const 0
              i32.load offset=3392
              local.tee 6
              i32.const 1
              local.get 4
              i32.shl
              local.tee 4
              i32.and
              br_if 0
              i32.const 0
              local.get 6
              local.get 4
              i32.or
              i32.store offset=3392
              local.get 0
              local.set 4
              br 1
            end
            local.get 0
            i32.load offset=8
            local.set 4
          end
          local.get 4
          local.get 5
          i32.store offset=12
          local.get 0
          local.get 5
          i32.store offset=8
          local.get 5
          local.get 0
          i32.store offset=12
          local.get 5
          local.get 4
          i32.store offset=8
          br 1
        end
        block
          block
            local.get 6
            i32.const 8
            i32.shr_u
            local.tee 4
            br_if 0
            i32.const 0
            local.set 0
            br 1
          end
          i32.const 31
          local.set 0
          local.get 6
          i32.const 16777215
          i32.gt_u
          br_if 0
          local.get 4
          local.get 4
          i32.const 1048320
          i32.add
          i32.const 16
          i32.shr_u
          i32.const 8
          i32.and
          local.tee 0
          i32.shl
          local.tee 4
          local.get 4
          i32.const 520192
          i32.add
          i32.const 16
          i32.shr_u
          i32.const 4
          i32.and
          local.tee 4
          i32.shl
          local.tee 3
          local.get 3
          i32.const 245760
          i32.add
          i32.const 16
          i32.shr_u
          i32.const 2
          i32.and
          local.tee 3
          i32.shl
          i32.const 15
          i32.shr_u
          local.get 4
          local.get 0
          i32.or
          local.get 3
          i32.or
          i32.sub
          local.tee 0
          i32.const 1
          i32.shl
          local.get 6
          local.get 0
          i32.const 21
          i32.add
          i32.shr_u
          i32.const 1
          i32.and
          i32.or
          i32.const 28
          i32.add
          local.set 0
        end
        local.get 5
        local.get 0
        i32.store offset=28
        local.get 5
        i64.const 0
        i64.store offset=16 align=4
        local.get 0
        i32.const 2
        i32.shl
        i32.const 3696
        i32.add
        local.set 4
        block
          local.get 7
          i32.const 1
          local.get 0
          i32.shl
          local.tee 3
          i32.and
          br_if 0
          local.get 4
          local.get 5
          i32.store
          i32.const 0
          local.get 7
          local.get 3
          i32.or
          i32.store offset=3396
          local.get 5
          local.get 4
          i32.store offset=24
          local.get 5
          local.get 5
          i32.store offset=8
          local.get 5
          local.get 5
          i32.store offset=12
          br 1
        end
        local.get 6
        i32.const 0
        i32.const 25
        local.get 0
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 0
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 0
        local.get 4
        i32.load
        local.set 3
        block
          loop
            local.get 3
            local.tee 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 6
            i32.eq
            br_if 1
            local.get 0
            i32.const 29
            i32.shr_u
            local.set 3
            local.get 0
            i32.const 1
            i32.shl
            local.set 0
            local.get 4
            local.get 3
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 2
            i32.load
            local.tee 3
            br_if 0
          end
          local.get 2
          local.get 5
          i32.store
          local.get 5
          local.get 4
          i32.store offset=24
          local.get 5
          local.get 5
          i32.store offset=12
          local.get 5
          local.get 5
          i32.store offset=8
          br 1
        end
        local.get 4
        i32.load offset=8
        local.set 0
        local.get 4
        local.get 5
        i32.store offset=8
        local.get 0
        local.get 5
        i32.store offset=12
        local.get 5
        i32.const 0
        i32.store offset=24
        local.get 5
        local.get 0
        i32.store offset=8
        local.get 5
        local.get 4
        i32.store offset=12
      end
      local.get 8
      i32.const 8
      i32.add
      local.set 0
      br 1
    end
    block
      local.get 10
      i32.eqz
      br_if 0
      block
        block
          local.get 5
          local.get 5
          i32.load offset=28
          local.tee 6
          i32.const 2
          i32.shl
          i32.const 3696
          i32.add
          local.tee 0
          i32.load
          i32.ne
          br_if 0
          local.get 0
          local.get 8
          i32.store
          local.get 8
          br_if 1
          i32.const 0
          local.get 9
          i32.const -2
          local.get 6
          i32.rotl
          i32.and
          i32.store offset=3396
          br 2
        end
        local.get 10
        i32.const 16
        i32.const 20
        local.get 10
        i32.load offset=16
        local.get 5
        i32.eq
        select
        i32.add
        local.get 8
        i32.store
        local.get 8
        i32.eqz
        br_if 1
      end
      local.get 8
      local.get 10
      i32.store offset=24
      block
        local.get 5
        i32.load offset=16
        local.tee 0
        i32.eqz
        br_if 0
        local.get 8
        local.get 0
        i32.store offset=16
        local.get 0
        local.get 8
        i32.store offset=24
      end
      local.get 5
      i32.const 20
      i32.add
      i32.load
      local.tee 0
      i32.eqz
      br_if 0
      local.get 8
      i32.const 20
      i32.add
      local.get 0
      i32.store
      local.get 0
      local.get 8
      i32.store offset=24
    end
    block
      block
        local.get 4
        i32.const 15
        i32.gt_u
        br_if 0
        local.get 5
        local.get 4
        local.get 3
        i32.add
        local.tee 0
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 5
        local.get 0
        i32.add
        local.tee 0
        local.get 0
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        br 1
      end
      local.get 5
      local.get 3
      i32.add
      local.tee 6
      local.get 4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 5
      local.get 3
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 6
      local.get 4
      i32.add
      local.get 4
      i32.store
      block
        local.get 7
        i32.eqz
        br_if 0
        local.get 7
        i32.const 3
        i32.shr_u
        local.tee 8
        i32.const 3
        i32.shl
        i32.const 3432
        i32.add
        local.set 3
        i32.const 0
        i32.load offset=3412
        local.set 0
        block
          block
            i32.const 1
            local.get 8
            i32.shl
            local.tee 8
            local.get 2
            i32.and
            br_if 0
            i32.const 0
            local.get 8
            local.get 2
            i32.or
            i32.store offset=3392
            local.get 3
            local.set 8
            br 1
          end
          local.get 3
          i32.load offset=8
          local.set 8
        end
        local.get 8
        local.get 0
        i32.store offset=12
        local.get 3
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 3
        i32.store offset=12
        local.get 0
        local.get 8
        i32.store offset=8
      end
      i32.const 0
      local.get 6
      i32.store offset=3412
      i32.const 0
      local.get 4
      i32.store offset=3400
    end
    local.get 5
    i32.const 8
    i32.add
    local.set 0
  end
  local.get 1
  i32.const 16
  i32.add
  global.set $mem
  local.get 0)
(func $free (param i32)
  local.get 0
  call $dlfree)
(func $dlfree (param i32)
  (local i32 i32 i32 i32 i32 i32 i32)
  block
    local.get 0
    i32.eqz
    br_if 0
    local.get 0
    i32.const -8
    i32.add
    local.tee 1
    local.get 0
    i32.const -4
    i32.add
    i32.load
    local.tee 2
    i32.const -8
    i32.and
    local.tee 0
    i32.add
    local.set 3
    block
      local.get 2
      i32.const 1
      i32.and
      br_if 0
      local.get 2
      i32.const 3
      i32.and
      i32.eqz
      br_if 1
      local.get 1
      local.get 1
      i32.load
      local.tee 2
      i32.sub
      local.tee 1
      i32.const 0
      i32.load offset=3408
      local.tee 4
      i32.lt_u
      br_if 1
      local.get 2
      local.get 0
      i32.add
      local.set 0
      block
        i32.const 0
        i32.load offset=3412
        local.get 1
        i32.eq
        br_if 0
        block
          local.get 2
          i32.const 255
          i32.gt_u
          br_if 0
          local.get 1
          i32.load offset=12
          local.set 5
          block
            local.get 1
            i32.load offset=8
            local.tee 6
            local.get 2
            i32.const 3
            i32.shr_u
            local.tee 7
            i32.const 3
            i32.shl
            i32.const 3432
            i32.add
            local.tee 2
            i32.eq
            br_if 0
            local.get 4
            local.get 6
            i32.gt_u
            drop
          end
          block
            local.get 5
            local.get 6
            i32.ne
            br_if 0
            i32.const 0
            i32.const 0
            i32.load offset=3392
            i32.const -2
            local.get 7
            i32.rotl
            i32.and
            i32.store offset=3392
            br 3
          end
          block
            local.get 5
            local.get 2
            i32.eq
            br_if 0
            local.get 4
            local.get 5
            i32.gt_u
            drop
          end
          local.get 5
          local.get 6
          i32.store offset=8
          local.get 6
          local.get 5
          i32.store offset=12
          br 2
        end
        local.get 1
        i32.load offset=24
        local.set 7
        block
          block
            local.get 1
            i32.load offset=12
            local.tee 5
            local.get 1
            i32.eq
            br_if 0
            block
              local.get 4
              local.get 1
              i32.load offset=8
              local.tee 2
              i32.gt_u
              br_if 0
              local.get 2
              i32.load offset=12
              local.get 1
              i32.ne
              drop
            end
            local.get 5
            local.get 2
            i32.store offset=8
            local.get 2
            local.get 5
            i32.store offset=12
            br 1
          end
          block
            local.get 1
            i32.const 20
            i32.add
            local.tee 2
            i32.load
            local.tee 4
            br_if 0
            local.get 1
            i32.const 16
            i32.add
            local.tee 2
            i32.load
            local.tee 4
            br_if 0
            i32.const 0
            local.set 5
            br 1
          end
          loop
            local.get 2
            local.set 6
            local.get 4
            local.tee 5
            i32.const 20
            i32.add
            local.tee 2
            i32.load
            local.tee 4
            br_if 0
            local.get 5
            i32.const 16
            i32.add
            local.set 2
            local.get 5
            i32.load offset=16
            local.tee 4
            br_if 0
          end
          local.get 6
          i32.const 0
          i32.store
        end
        local.get 7
        i32.eqz
        br_if 1
        block
          block
            local.get 1
            i32.load offset=28
            local.tee 4
            i32.const 2
            i32.shl
            i32.const 3696
            i32.add
            local.tee 2
            i32.load
            local.get 1
            i32.ne
            br_if 0
            local.get 2
            local.get 5
            i32.store
            local.get 5
            br_if 1
            i32.const 0
            i32.const 0
            i32.load offset=3396
            i32.const -2
            local.get 4
            i32.rotl
            i32.and
            i32.store offset=3396
            br 3
          end
          local.get 7
          i32.const 16
          i32.const 20
          local.get 7
          i32.load offset=16
          local.get 1
          i32.eq
          select
          i32.add
          local.get 5
          i32.store
          local.get 5
          i32.eqz
          br_if 2
        end
        local.get 5
        local.get 7
        i32.store offset=24
        block
          local.get 1
          i32.load offset=16
          local.tee 2
          i32.eqz
          br_if 0
          local.get 5
          local.get 2
          i32.store offset=16
          local.get 2
          local.get 5
          i32.store offset=24
        end
        local.get 1
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 1
        local.get 5
        i32.const 20
        i32.add
        local.get 2
        i32.store
        local.get 2
        local.get 5
        i32.store offset=24
        br 1
      end
      local.get 3
      i32.load offset=4
      local.tee 2
      i32.const 3
      i32.and
      i32.const 3
      i32.ne
      br_if 0
      local.get 3
      local.get 2
      i32.const -2
      i32.and
      i32.store offset=4
      i32.const 0
      local.get 0
      i32.store offset=3400
      local.get 1
      local.get 0
      i32.add
      local.get 0
      i32.store
      local.get 1
      local.get 0
      i32.const 1
      i32.or
      i32.store offset=4
      return
    end
    local.get 3
    local.get 1
    i32.le_u
    br_if 0
    local.get 3
    i32.load offset=4
    local.tee 2
    i32.const 1
    i32.and
    i32.eqz
    br_if 0
    block
      block
        local.get 2
        i32.const 2
        i32.and
        br_if 0
        block
          i32.const 0
          i32.load offset=3416
          local.get 3
          i32.ne
          br_if 0
          i32.const 0
          local.get 1
          i32.store offset=3416
          i32.const 0
          i32.const 0
          i32.load offset=3404
          local.get 0
          i32.add
          local.tee 0
          i32.store offset=3404
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          i32.const 0
          i32.load offset=3412
          i32.ne
          br_if 3
          i32.const 0
          i32.const 0
          i32.store offset=3400
          i32.const 0
          i32.const 0
          i32.store offset=3412
          return
        end
        block
          i32.const 0
          i32.load offset=3412
          local.get 3
          i32.ne
          br_if 0
          i32.const 0
          local.get 1
          i32.store offset=3412
          i32.const 0
          i32.const 0
          i32.load offset=3400
          local.get 0
          i32.add
          local.tee 0
          i32.store offset=3400
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          return
        end
        local.get 2
        i32.const -8
        i32.and
        local.get 0
        i32.add
        local.set 0
        block
          block
            local.get 2
            i32.const 255
            i32.gt_u
            br_if 0
            local.get 3
            i32.load offset=12
            local.set 4
            block
              local.get 3
              i32.load offset=8
              local.tee 5
              local.get 2
              i32.const 3
              i32.shr_u
              local.tee 3
              i32.const 3
              i32.shl
              i32.const 3432
              i32.add
              local.tee 2
              i32.eq
              br_if 0
              i32.const 0
              i32.load offset=3408
              local.get 5
              i32.gt_u
              drop
            end
            block
              local.get 4
              local.get 5
              i32.ne
              br_if 0
              i32.const 0
              i32.const 0
              i32.load offset=3392
              i32.const -2
              local.get 3
              i32.rotl
              i32.and
              i32.store offset=3392
              br 2
            end
            block
              local.get 4
              local.get 2
              i32.eq
              br_if 0
              i32.const 0
              i32.load offset=3408
              local.get 4
              i32.gt_u
              drop
            end
            local.get 4
            local.get 5
            i32.store offset=8
            local.get 5
            local.get 4
            i32.store offset=12
            br 1
          end
          local.get 3
          i32.load offset=24
          local.set 7
          block
            block
              local.get 3
              i32.load offset=12
              local.tee 5
              local.get 3
              i32.eq
              br_if 0
              block
                i32.const 0
                i32.load offset=3408
                local.get 3
                i32.load offset=8
                local.tee 2
                i32.gt_u
                br_if 0
                local.get 2
                i32.load offset=12
                local.get 3
                i32.ne
                drop
              end
              local.get 5
              local.get 2
              i32.store offset=8
              local.get 2
              local.get 5
              i32.store offset=12
              br 1
            end
            block
              local.get 3
              i32.const 20
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0
              local.get 3
              i32.const 16
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0
              i32.const 0
              local.set 5
              br 1
            end
            loop
              local.get 2
              local.set 6
              local.get 4
              local.tee 5
              i32.const 20
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0
              local.get 5
              i32.const 16
              i32.add
              local.set 2
              local.get 5
              i32.load offset=16
              local.tee 4
              br_if 0
            end
            local.get 6
            i32.const 0
            i32.store
          end
          local.get 7
          i32.eqz
          br_if 0
          block
            block
              local.get 3
              i32.load offset=28
              local.tee 4
              i32.const 2
              i32.shl
              i32.const 3696
              i32.add
              local.tee 2
              i32.load
              local.get 3
              i32.ne
              br_if 0
              local.get 2
              local.get 5
              i32.store
              local.get 5
              br_if 1
              i32.const 0
              i32.const 0
              i32.load offset=3396
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              i32.store offset=3396
              br 2
            end
            local.get 7
            i32.const 16
            i32.const 20
            local.get 7
            i32.load offset=16
            local.get 3
            i32.eq
            select
            i32.add
            local.get 5
            i32.store
            local.get 5
            i32.eqz
            br_if 1
          end
          local.get 5
          local.get 7
          i32.store offset=24
          block
            local.get 3
            i32.load offset=16
            local.tee 2
            i32.eqz
            br_if 0
            local.get 5
            local.get 2
            i32.store offset=16
            local.get 2
            local.get 5
            i32.store offset=24
          end
          local.get 3
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0
          local.get 5
          i32.const 20
          i32.add
          local.get 2
          i32.store
          local.get 2
          local.get 5
          i32.store offset=24
        end
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        i32.const 0
        i32.load offset=3412
        i32.ne
        br_if 1
        i32.const 0
        local.get 0
        i32.store offset=3400
        return
      end
      local.get 3
      local.get 2
      i32.const -2
      i32.and
      i32.store offset=4
      local.get 1
      local.get 0
      i32.add
      local.get 0
      i32.store
      local.get 1
      local.get 0
      i32.const 1
      i32.or
      i32.store offset=4
    end
    block
      local.get 0
      i32.const 255
      i32.gt_u
      br_if 0
      local.get 0
      i32.const 3
      i32.shr_u
      local.tee 2
      i32.const 3
      i32.shl
      i32.const 3432
      i32.add
      local.set 0
      block
        block
          i32.const 0
          i32.load offset=3392
          local.tee 4
          i32.const 1
          local.get 2
          i32.shl
          local.tee 2
          i32.and
          br_if 0
          i32.const 0
          local.get 4
          local.get 2
          i32.or
          i32.store offset=3392
          local.get 0
          local.set 2
          br 1
        end
        local.get 0
        i32.load offset=8
        local.set 2
      end
      local.get 2
      local.get 1
      i32.store offset=12
      local.get 0
      local.get 1
      i32.store offset=8
      local.get 1
      local.get 0
      i32.store offset=12
      local.get 1
      local.get 2
      i32.store offset=8
      return
    end
    i32.const 0
    local.set 2
    block
      local.get 0
      i32.const 8
      i32.shr_u
      local.tee 4
      i32.eqz
      br_if 0
      i32.const 31
      local.set 2
      local.get 0
      i32.const 16777215
      i32.gt_u
      br_if 0
      local.get 4
      local.get 4
      i32.const 1048320
      i32.add
      i32.const 16
      i32.shr_u
      i32.const 8
      i32.and
      local.tee 2
      i32.shl
      local.tee 4
      local.get 4
      i32.const 520192
      i32.add
      i32.const 16
      i32.shr_u
      i32.const 4
      i32.and
      local.tee 4
      i32.shl
      local.tee 5
      local.get 5
      i32.const 245760
      i32.add
      i32.const 16
      i32.shr_u
      i32.const 2
      i32.and
      local.tee 5
      i32.shl
      i32.const 15
      i32.shr_u
      local.get 4
      local.get 2
      i32.or
      local.get 5
      i32.or
      i32.sub
      local.tee 2
      i32.const 1
      i32.shl
      local.get 0
      local.get 2
      i32.const 21
      i32.add
      i32.shr_u
      i32.const 1
      i32.and
      i32.or
      i32.const 28
      i32.add
      local.set 2
    end
    local.get 1
    i64.const 0
    i64.store offset=16 align=4
    local.get 1
    i32.const 28
    i32.add
    local.get 2
    i32.store
    local.get 2
    i32.const 2
    i32.shl
    i32.const 3696
    i32.add
    local.set 4
    block
      block
        i32.const 0
        i32.load offset=3396
        local.tee 5
        i32.const 1
        local.get 2
        i32.shl
        local.tee 3
        i32.and
        br_if 0
        local.get 4
        local.get 1
        i32.store
        i32.const 0
        local.get 5
        local.get 3
        i32.or
        i32.store offset=3396
        local.get 1
        i32.const 24
        i32.add
        local.get 4
        i32.store
        local.get 1
        local.get 1
        i32.store offset=8
        local.get 1
        local.get 1
        i32.store offset=12
        br 1
      end
      local.get 0
      i32.const 0
      i32.const 25
      local.get 2
      i32.const 1
      i32.shr_u
      i32.sub
      local.get 2
      i32.const 31
      i32.eq
      select
      i32.shl
      local.set 2
      local.get 4
      i32.load
      local.set 5
      block
        loop
          local.get 5
          local.tee 4
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 0
          i32.eq
          br_if 1
          local.get 2
          i32.const 29
          i32.shr_u
          local.set 5
          local.get 2
          i32.const 1
          i32.shl
          local.set 2
          local.get 4
          local.get 5
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 3
          i32.load
          local.tee 5
          br_if 0
        end
        local.get 3
        local.get 1
        i32.store
        local.get 1
        local.get 1
        i32.store offset=12
        local.get 1
        i32.const 24
        i32.add
        local.get 4
        i32.store
        local.get 1
        local.get 1
        i32.store offset=8
        br 1
      end
      local.get 4
      i32.load offset=8
      local.set 0
      local.get 4
      local.get 1
      i32.store offset=8
      local.get 0
      local.get 1
      i32.store offset=12
      local.get 1
      i32.const 24
      i32.add
      i32.const 0
      i32.store
      local.get 1
      local.get 0
      i32.store offset=8
      local.get 1
      local.get 4
      i32.store offset=12
    end
    i32.const 0
    i32.const 0
    i32.load offset=3424
    i32.const -1
    i32.add
    local.tee 1
    i32.store offset=3424
    local.get 1
    br_if 0
    i32.const 3848
    local.set 1
    loop
      local.get 1
      i32.load
      local.tee 0
      i32.const 8
      i32.add
      local.set 1
      local.get 0
      br_if 0
    end
    i32.const 0
    i32.const -1
    i32.store offset=3424
  end)
(func $sbrk (param i32) (result i32)
  block
    local.get 0
    br_if 0
    memory.size
    i32.const 16
    i32.shl
    return
  end
  block
    local.get 0
    i32.const 65535
    i32.and
    br_if 0
    local.get 0
    i32.const -1
    i32.le_s
    br_if 0
    block
      local.get 0
      i32.const 16
      i32.shr_u
      memory.grow
      local.tee 0
      i32.const -1
      i32.ne
      br_if 0
      i32.const 0
      i32.const 48
      i32.store offset=3888
      i32.const -1
      return
    end
    local.get 0
    i32.const 16
    i32.shl
    return
  end
  call $abort
  unreachable)
(func $abort
  unreachable
  unreachable)
(global $mem (mut i32) (i32.const 70752))
(func $_start
call $main
call $_exit)
(memory 2)
(export "memory" (memory 0))
(export "_start" (func $_start)))