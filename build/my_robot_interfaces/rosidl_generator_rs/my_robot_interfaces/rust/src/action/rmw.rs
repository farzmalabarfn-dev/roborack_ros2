
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_Goal() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__CountUntil_Goal__init(msg: *mut CountUntil_Goal) -> bool;
    fn my_robot_interfaces__action__CountUntil_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_Goal>, size: usize) -> bool;
    fn my_robot_interfaces__action__CountUntil_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_Goal>);
    fn my_robot_interfaces__action__CountUntil_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CountUntil_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<CountUntil_Goal>) -> bool;
}

// Corresponds to my_robot_interfaces__action__CountUntil_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CountUntil_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target_count: i32,

}



impl Default for CountUntil_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__CountUntil_Goal__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__CountUntil_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CountUntil_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CountUntil_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CountUntil_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/CountUntil_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_Goal() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_Result() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__CountUntil_Result__init(msg: *mut CountUntil_Result) -> bool;
    fn my_robot_interfaces__action__CountUntil_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_Result>, size: usize) -> bool;
    fn my_robot_interfaces__action__CountUntil_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_Result>);
    fn my_robot_interfaces__action__CountUntil_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CountUntil_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<CountUntil_Result>) -> bool;
}

// Corresponds to my_robot_interfaces__action__CountUntil_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CountUntil_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for CountUntil_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__CountUntil_Result__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__CountUntil_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CountUntil_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CountUntil_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CountUntil_Result where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/CountUntil_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_Result() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__CountUntil_Feedback__init(msg: *mut CountUntil_Feedback) -> bool;
    fn my_robot_interfaces__action__CountUntil_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_Feedback>, size: usize) -> bool;
    fn my_robot_interfaces__action__CountUntil_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_Feedback>);
    fn my_robot_interfaces__action__CountUntil_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CountUntil_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<CountUntil_Feedback>) -> bool;
}

// Corresponds to my_robot_interfaces__action__CountUntil_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CountUntil_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_number: i32,

}



impl Default for CountUntil_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__CountUntil_Feedback__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__CountUntil_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CountUntil_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CountUntil_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CountUntil_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/CountUntil_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_Feedback() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__CountUntil_FeedbackMessage__init(msg: *mut CountUntil_FeedbackMessage) -> bool;
    fn my_robot_interfaces__action__CountUntil_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_FeedbackMessage>, size: usize) -> bool;
    fn my_robot_interfaces__action__CountUntil_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_FeedbackMessage>);
    fn my_robot_interfaces__action__CountUntil_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CountUntil_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<CountUntil_FeedbackMessage>) -> bool;
}

// Corresponds to my_robot_interfaces__action__CountUntil_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CountUntil_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::CountUntil_Feedback,

}



impl Default for CountUntil_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__CountUntil_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__CountUntil_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CountUntil_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CountUntil_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CountUntil_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/CountUntil_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_FeedbackMessage() }
  }
}




#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__CountUntil_SendGoal_Request__init(msg: *mut CountUntil_SendGoal_Request) -> bool;
    fn my_robot_interfaces__action__CountUntil_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_SendGoal_Request>, size: usize) -> bool;
    fn my_robot_interfaces__action__CountUntil_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_SendGoal_Request>);
    fn my_robot_interfaces__action__CountUntil_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CountUntil_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CountUntil_SendGoal_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__action__CountUntil_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CountUntil_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::CountUntil_Goal,

}



impl Default for CountUntil_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__CountUntil_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__CountUntil_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CountUntil_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CountUntil_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CountUntil_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/CountUntil_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_SendGoal_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__CountUntil_SendGoal_Response__init(msg: *mut CountUntil_SendGoal_Response) -> bool;
    fn my_robot_interfaces__action__CountUntil_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_SendGoal_Response>, size: usize) -> bool;
    fn my_robot_interfaces__action__CountUntil_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_SendGoal_Response>);
    fn my_robot_interfaces__action__CountUntil_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CountUntil_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CountUntil_SendGoal_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__action__CountUntil_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CountUntil_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for CountUntil_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__CountUntil_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__CountUntil_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CountUntil_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CountUntil_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CountUntil_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/CountUntil_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_SendGoal_Response() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__CountUntil_GetResult_Request__init(msg: *mut CountUntil_GetResult_Request) -> bool;
    fn my_robot_interfaces__action__CountUntil_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_GetResult_Request>, size: usize) -> bool;
    fn my_robot_interfaces__action__CountUntil_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_GetResult_Request>);
    fn my_robot_interfaces__action__CountUntil_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CountUntil_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CountUntil_GetResult_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__action__CountUntil_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CountUntil_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for CountUntil_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__CountUntil_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__CountUntil_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CountUntil_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CountUntil_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CountUntil_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/CountUntil_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_GetResult_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__action__CountUntil_GetResult_Response__init(msg: *mut CountUntil_GetResult_Response) -> bool;
    fn my_robot_interfaces__action__CountUntil_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_GetResult_Response>, size: usize) -> bool;
    fn my_robot_interfaces__action__CountUntil_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CountUntil_GetResult_Response>);
    fn my_robot_interfaces__action__CountUntil_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CountUntil_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CountUntil_GetResult_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__action__CountUntil_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CountUntil_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::CountUntil_Result,

}



impl Default for CountUntil_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__action__CountUntil_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__action__CountUntil_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CountUntil_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__action__CountUntil_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CountUntil_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CountUntil_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/action/CountUntil_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__action__CountUntil_GetResult_Response() }
  }
}






#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__CountUntil_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__action__CountUntil_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct CountUntil_SendGoal;

impl rosidl_runtime_rs::Service for CountUntil_SendGoal {
    type Request = CountUntil_SendGoal_Request;
    type Response = CountUntil_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__CountUntil_SendGoal() }
    }
}




#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__CountUntil_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__action__CountUntil_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct CountUntil_GetResult;

impl rosidl_runtime_rs::Service for CountUntil_GetResult {
    type Request = CountUntil_GetResult_Request;
    type Response = CountUntil_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__action__CountUntil_GetResult() }
    }
}


