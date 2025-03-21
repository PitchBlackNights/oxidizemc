use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jint, jstring};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_pitchblacknights_oxidizemc_OxidizeMC_returnTest(
    env: JNIEnv,
    _class: JClass,
    num: jint,
) -> jstring {
    let i = num as i32;
    let output = env
        .new_string(format!("Hello, {}!", i))
        .expect("Couldn't create java string!");

    output.as_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_pitchblacknights_oxidizemc_OxidizeMC_testLogString(
    mut env: JNIEnv,
    class: JClass,
    text: JString,
) {
    let input: String = env
        .get_string(&text)
        .expect("[RUST_LIB] [testLogString()] Error when getting java string from function argument `text`")
        .into();
    let output: JString<'_> = env
        .new_string(format!("LOGGED FROM RUST  -  {}", input))
        .expect("[RUST_LIB] [testLogString()] Error when creating java string from variable `input`");

    let logger: JObject<'_> = env
        .get_static_field(class, "LOGGER", "Lorg/slf4j/Logger;")
        .expect("[RUST_LIB] [testLogString()] Error when getting static field `LOGGER`")
        .l()
        .unwrap();

    // Apparently JNI doesn't know crap about the errors it gives you
    // So you just gotta "handle" the error & finish the function (or return early) to return back to Java
    // After that, Java will print out the Error for you

    // FUNCTION ERRORS WITH: `java.lang.NoSuchMethodError: info`
    // Don't know whats goin on with that.
    // Supposed to call `LOGGER.info()`, but might just be calling `info()`?
    match env.call_method(
        logger,
        "info",
        "(Ljava/lang/String;)V;",
        &[JValue::Object(&JObject::from(output))],
    ) {
        Ok(val) => val.v().unwrap(),
        Err(err) => println!("[RUST_LIB] [testLogString()] Error when calling method `LOGGER.info()`: {}", err),
    }
}
