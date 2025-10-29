///interface [ClientModInitializer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/ClientModInitializer.html)
pub enum ClientModInitializer {}
unsafe impl ::java_spaghetti::ReferenceType for ClientModInitializer {}
unsafe impl ::java_spaghetti::JniType for ClientModInitializer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/ClientModInitializer")
    }
}
impl ClientModInitializer {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/ClientModInitializer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[onInitializeClient](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/ClientModInitializer.html#onInitializeClient())
    pub fn onInitializeClient<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        (),
        ::java_spaghetti::Local<'env, super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"onInitializeClient",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [DedicatedServerModInitializer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/DedicatedServerModInitializer.html)
pub enum DedicatedServerModInitializer {}
unsafe impl ::java_spaghetti::ReferenceType for DedicatedServerModInitializer {}
unsafe impl ::java_spaghetti::JniType for DedicatedServerModInitializer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/DedicatedServerModInitializer")
    }
}
impl DedicatedServerModInitializer {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/DedicatedServerModInitializer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[onInitializeServer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/DedicatedServerModInitializer.html#onInitializeServer())
    pub fn onInitializeServer<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        (),
        ::java_spaghetti::Local<'env, super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"onInitializeServer",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [EnvType](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/EnvType.html)
pub enum EnvType {}
unsafe impl ::java_spaghetti::ReferenceType for EnvType {}
unsafe impl ::java_spaghetti::JniType for EnvType {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/EnvType")
    }
}
impl EnvType {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/EnvType"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/EnvType.html#values())
    pub fn values<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_spaghetti::Local<
                'env,
                ::java_spaghetti::ObjectArray<EnvType, super::super::super::java::lang::Throwable>,
            >,
        >,
        ::java_spaghetti::Local<'env, super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"values",
                        c"()[Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [CLIENT](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/EnvType.html#CLIENT)
    pub fn CLIENT<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, EnvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"CLIENT",
                        c"Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [SERVER](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/EnvType.html#SERVER)
    pub fn SERVER<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, EnvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"SERVER",
                        c"Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///interface [Environment](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/Environment.html)
pub enum Environment {}
unsafe impl ::java_spaghetti::ReferenceType for Environment {}
unsafe impl ::java_spaghetti::JniType for Environment {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/Environment")
    }
}
impl Environment {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/Environment"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[value](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/Environment.html#value())
    pub fn value<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, EnvType>>,
        ::java_spaghetti::Local<'env, super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"value",
                        c"()Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [EnvironmentInterface](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/EnvironmentInterface.html)
pub enum EnvironmentInterface {}
unsafe impl ::java_spaghetti::ReferenceType for EnvironmentInterface {}
unsafe impl ::java_spaghetti::JniType for EnvironmentInterface {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/EnvironmentInterface")
    }
}
impl EnvironmentInterface {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/EnvironmentInterface"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[value](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/EnvironmentInterface.html#value())
    pub fn value<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, EnvType>>,
        ::java_spaghetti::Local<'env, super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"value",
                        c"()Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [EnvironmentInterfaces](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/EnvironmentInterfaces.html)
pub enum EnvironmentInterfaces {}
unsafe impl ::java_spaghetti::ReferenceType for EnvironmentInterfaces {}
unsafe impl ::java_spaghetti::JniType for EnvironmentInterfaces {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/EnvironmentInterfaces")
    }
}
impl EnvironmentInterfaces {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/EnvironmentInterfaces"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[value](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/EnvironmentInterfaces.html#value())
    pub fn value<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_spaghetti::Local<
                'env,
                ::java_spaghetti::ObjectArray<
                    EnvironmentInterface,
                    super::super::super::java::lang::Throwable,
                >,
            >,
        >,
        ::java_spaghetti::Local<'env, super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"value",
                        c"()[Lnet/fabricmc/api/EnvironmentInterface;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [ModInitializer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/ModInitializer.html)
pub enum ModInitializer {}
unsafe impl ::java_spaghetti::ReferenceType for ModInitializer {}
unsafe impl ::java_spaghetti::JniType for ModInitializer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/ModInitializer")
    }
}
impl ModInitializer {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/ModInitializer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[onInitialize](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/api/ModInitializer.html#onInitialize())
    pub fn onInitialize<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        (),
        ::java_spaghetti::Local<'env, super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"onInitialize",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
