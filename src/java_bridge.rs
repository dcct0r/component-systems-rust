use actix_web::{HttpResponse, Responder};
use jni::errors::Result;
use jni::objects::{JObject, JString, JValue};
use jni::{AttachGuard, Executor, JNIEnv, JavaVM};
use std::sync::{Arc, Once};

static INIT: Once = Once::new();
static mut JVM: Option<Arc<JavaVM>> = None;

pub fn init_jvm(jvm: JavaVM) {
    INIT.call_once(|| unsafe {
        JVM = Some(Arc::new(jvm));
    });
}

pub fn get_jvm() -> Arc<JavaVM> {
    unsafe {
        JVM.as_ref()
            .expect("JVM not initialized. Call JNI_OnLoad first.")
            .clone()
    }
}

pub(crate) fn attach_env(jvm: &Arc<JavaVM>) -> Result<AttachGuard> {
    let jvm_ref: &Arc<JavaVM> = &*jvm;
    jvm_ref.attach_current_thread()
}

pub struct JniService {
    jvm: Arc<JavaVM>,
}

impl JniService {
    pub fn from_env(env: &JNIEnv) -> Result<Self> {
        Ok(Self {
            jvm: Arc::new(env.get_java_vm()?),
        })
    }

    fn get_env<'local>(&self) -> Result<AttachGuard> {
        attach_env(&self.jvm)
    }

    pub unsafe fn create_incident(
        &self,
        title: &str,
        description: &str,
        priority: &str,
    ) -> Result<String> {
        let mut env = self.get_env()?;
        let cls = env.find_class("com/incident/jni/IncidentServiceFacade")?;

        let title_j = env.new_string(title)?;
        let desc_j = env.new_string(description)?;
        let priority_j = env.new_string(priority)?;

        // Ссылки на JObject
        let title_obj = JObject::from_raw(**title_j);
        let desc_obj = JObject::from_raw(**desc_j);
        let priority_obj = JObject::from_raw(**priority_j);

        // Вызов статического метода Java
        let res = env
            .call_static_method(
                cls,
                "createIncident",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                &[
                    JValue::Object(&title_obj),
                    JValue::Object(&desc_obj),
                    JValue::Object(&priority_obj),
                ],
            )?
            .l()?;

        let jstr = JString::from_raw(*res);
        env.get_string(&jstr).map(|s| s.into())
    }

    pub unsafe fn change_status(
        &self,
        id: &str,
        status: &str,
        assignee: &str,
        comment: &str,
    ) -> Result<String> {
        let mut env = self.get_env()?;
        let cls = env.find_class("com/incident/jni/IncidentServiceFacade")?;

        let id_j = env.new_string(id)?;
        let status_j = env.new_string(status)?;
        let assignee_j = env.new_string(assignee)?;
        let comment_j = env.new_string(comment)?;

        let id_obj = JObject::from_raw(**id_j);
        let status_obj = JObject::from_raw(**status_j);
        let assignee_obj = JObject::from_raw(**assignee_j);
        let comment_obj = JObject::from_raw(**comment_j);

        let res = env
            .call_static_method(
                cls,
                "changeStatus",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                &[
                    JValue::Object(&id_obj),
                    JValue::Object(&status_obj),
                    JValue::Object(&assignee_obj),
                    JValue::Object(&comment_obj),
                ],
            )?
            .l()?;

        let jstr = JString::from_raw(*res);
        env.get_string(&jstr).map(|s| s.into())
    }
}

pub struct JniExecutor {
    executor: Executor,
}

impl JniExecutor {
    pub fn from_env(env: &AttachGuard) -> Result<Self> {
        let jvm = env.get_java_vm()?;
        Ok(Self {
            executor: Executor::new(jvm.into()),
        })
    }

    pub fn create_incident(
        &self,
        title: &str,
        description: &str,
        priority: &str,
    ) -> Result<String> {
        self.executor.with_attached(|env| unsafe {
            let cls = env.find_class("com/incident/jni/IncidentServiceFacade")?;
            let title_j = env.new_string(title)?;
            let desc_j = env.new_string(description)?;
            let priority_j = env.new_string(priority)?;

            let title_obj = JObject::from_raw(**title_j);
            let desc_obj = JObject::from_raw(**desc_j);
            let priority_obj = JObject::from_raw(**priority_j);

            let res = env
                .call_static_method(
                    cls,
                    "createIncident",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    &[
                        JValue::Object(&title_obj),
                        JValue::Object(&desc_obj),
                        JValue::Object(&priority_obj),
                    ],
                )?
                .l()?;

            let jstr = JString::from_raw(*res);
            let x = Ok(env.get_string(&jstr)?.into());
            x
        })
    }

    pub fn change_status(
        &self,
        id: &str,
        status: &str,
        assignee: &str,
        comment: &str,
    ) -> Result<String> {
        self.executor.with_attached(|env| unsafe {
            let cls = env.find_class("com/incident/jni/IncidentServiceFacade")?;
            let id_j = env.new_string(id)?;
            let status_j = env.new_string(status)?;
            let assignee_j = env.new_string(assignee)?;
            let comment_j = env.new_string(comment)?;

            let id_obj = JObject::from_raw(**id_j);
            let status_obj = JObject::from_raw(**status_j);
            let assignee_obj = JObject::from_raw(**assignee_j);
            let comment_obj = JObject::from_raw(**comment_j);

            let res = env
                .call_static_method(
                    cls,
                    "changeStatus",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    &[
                        JValue::Object(&id_obj),
                        JValue::Object(&status_obj),
                        JValue::Object(&assignee_obj),
                        JValue::Object(&comment_obj),
                    ],
                )?
                .l()?;

            let jstr = JString::from_raw(*res);
            let x = Ok(env.get_string(&jstr)?.into());
            x
        })
    }
}

pub async fn create_incident_endpoint(
    title: &str,
    description: &str,
    priority: &str,
) -> impl Responder {
    let jvm = get_jvm();

    let env = match attach_env(&jvm) {
        Ok(env) => env,
        Err(err) => return HttpResponse::InternalServerError().body(format!("JNI Error: {}", err)),
    };

    let executor = match JniExecutor::from_env(&env) {
        Ok(ex) => ex,
        Err(err) => return HttpResponse::InternalServerError().body(format!("JNI Error: {}", err)),
    };

    match executor.create_incident(title, description, priority) {
        Ok(res) => HttpResponse::Ok().body(res),
        Err(err) => HttpResponse::InternalServerError().body(format!("JNI Error: {}", err)),
    }
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    vm: JavaVM,
    _reserved: *mut std::os::raw::c_void,
) -> jni::sys::jint {
    init_jvm(vm);
    jni::sys::JNI_VERSION_1_8
}
