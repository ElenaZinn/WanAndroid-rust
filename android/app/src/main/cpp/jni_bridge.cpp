#include <jni.h>
#include <string>
#include <cstdlib>

extern "C" {
struct NativeStringResult { char* value; char* error; };
void wanandroid_string_free(char* value);
void* wanandroid_core_create(unsigned long long timeout_ms);
void wanandroid_core_destroy(void* handle);
NativeStringResult wanandroid_core_snapshot(const void* handle);
NativeStringResult wanandroid_core_dispatch(const void* handle, const char* action_json);
}

static jstring result_to_jstring(JNIEnv* env, NativeStringResult result) {
    char* value = result.value != nullptr ? result.value : result.error;
    jstring output = env->NewStringUTF(value != nullptr ? value : "native core returned no result");
    wanandroid_string_free(value);
    return output;
}

extern "C" JNIEXPORT jlong JNICALL
Java_com_elena_wanandroidrust_rust_JniRustCoreBinding_nativeCreate(
    JNIEnv*, jobject, jlong timeout_ms) {
    return reinterpret_cast<jlong>(wanandroid_core_create(static_cast<unsigned long long>(timeout_ms)));
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_elena_wanandroidrust_rust_JniRustCoreBinding_nativeSnapshot(
    JNIEnv* env, jobject, jlong handle) {
    return result_to_jstring(env, wanandroid_core_snapshot(reinterpret_cast<const void*>(handle)));
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_elena_wanandroidrust_rust_JniRustCoreBinding_nativeDispatch(
    JNIEnv* env, jobject, jlong handle, jstring action_json) {
    const char* action = env->GetStringUTFChars(action_json, nullptr);
    NativeStringResult result = wanandroid_core_dispatch(reinterpret_cast<const void*>(handle), action);
    env->ReleaseStringUTFChars(action_json, action);
    return result_to_jstring(env, result);
}

extern "C" JNIEXPORT void JNICALL
Java_com_elena_wanandroidrust_rust_JniRustCoreBinding_nativeDestroy(
    JNIEnv*, jobject, jlong handle) {
    wanandroid_core_destroy(reinterpret_cast<void*>(handle));
}
