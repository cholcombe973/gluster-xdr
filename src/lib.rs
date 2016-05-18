extern crate xdr_codec;
// GENERATED CODE
//
// Generated from /home/chris/repos/glusterfs/rpc/xdr/src/everything.x.
//
// DO NOT EDIT


pub const LM_MAXSTRLEN: i64 = 1024i64;

pub const MAXNAMELEN: i64 = 1025i64;

pub const MAXNETOBJ_SZ: i64 = 1024i64;

pub const MNTUDPPATHLEN: i64 = 1024i64;

pub const SM_MAXSTRLEN: i64 = 1024i64;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct aclentry {
    pub acl_type: i32,
    pub uid: i32,
    pub perm: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct auth_glusterfs_parms {
    pub lk_owner: i64,
    pub pid: u32,
    pub uid: u32,
    pub gid: u32,
    pub ngrps: u32,
    pub groups: [u32; 16i64 as usize],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct auth_glusterfs_parms_v2 {
    pub pid: i32,
    pub uid: u32,
    pub gid: u32,
    pub groups: Vec<u32>,
    pub lk_owner: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct changelog_event_req {
    pub seq: u64,
    pub tv_sec: u64,
    pub tv_usec: u64,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct changelog_event_rsp {
    pub op_ret: i32,
    pub seq: u64,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct changelog_probe_rsp {
    pub op_ret: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum fsh_access {
    fsa_NONE = 0isize,
    fsa_R = 1isize,
    fsa_W = 2isize,
    fsa_RW = 3isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum fsh_mode {
    fsm_DN = 0isize,
    fsm_DR = 1isize,
    fsm_DW = 2isize,
    fsm_DRW = 3isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_brick_op_req {
    pub name: String,
    pub op: i32,
    pub input: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_brick_op_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub output: Vec<u8>,
    pub op_errstr: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gd1_mgmt_cluster_lock_req {
    pub uuid: [u8; 16i64 as usize],
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gd1_mgmt_cluster_lock_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub op_ret: i32,
    pub op_errno: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gd1_mgmt_cluster_unlock_req {
    pub uuid: [u8; 16i64 as usize],
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gd1_mgmt_cluster_unlock_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub op_ret: i32,
    pub op_errno: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_commit_op_req {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub buf: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_commit_op_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub op_ret: i32,
    pub op_errno: i32,
    pub dict: Vec<u8>,
    pub op_errstr: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_friend_req {
    pub uuid: [u8; 16i64 as usize],
    pub hostname: String,
    pub port: i32,
    pub vols: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_friend_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub hostname: String,
    pub op_ret: i32,
    pub op_errno: i32,
    pub port: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_friend_update {
    pub uuid: [u8; 16i64 as usize],
    pub friends: Vec<u8>,
    pub port: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gd1_mgmt_friend_update_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub op_ret: i32,
    pub op_errno: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_probe_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub hostname: String,
    pub port: i32,
    pub op_ret: i32,
    pub op_errno: i32,
    pub op_errstr: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_stage_op_req {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub buf: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_stage_op_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub op_ret: i32,
    pub op_errno: i32,
    pub op_errstr: String,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_unfriend_req {
    pub uuid: [u8; 16i64 as usize],
    pub hostname: String,
    pub port: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_unfriend_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub hostname: String,
    pub op_ret: i32,
    pub op_errno: i32,
    pub port: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_brick_op_req {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_brick_op_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub op_ret: i32,
    pub op_errno: i32,
    pub op_errstr: String,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_commit_req {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_commit_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub op_ret: i32,
    pub op_errno: i32,
    pub dict: Vec<u8>,
    pub op_errstr: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_lock_req {
    pub uuid: [u8; 16i64 as usize],
    pub txn_id: [u8; 16i64 as usize],
    pub op: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_lock_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub txn_id: [u8; 16i64 as usize],
    pub dict: Vec<u8>,
    pub op_ret: i32,
    pub op_errno: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_post_val_req {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub op_ret: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_post_val_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub op_ret: i32,
    pub op_errno: i32,
    pub op_errstr: String,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_pre_val_req {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_pre_val_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub op: i32,
    pub op_ret: i32,
    pub op_errno: i32,
    pub op_errstr: String,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_unlock_req {
    pub uuid: [u8; 16i64 as usize],
    pub txn_id: [u8; 16i64 as usize],
    pub op: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gd1_mgmt_v3_unlock_rsp {
    pub uuid: [u8; 16i64 as usize],
    pub txn_id: [u8; 16i64 as usize],
    pub dict: Vec<u8>,
    pub op_ret: i32,
    pub op_errno: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_friends_list {
    GF_CLI_LIST_PEERS = 1isize,
    GF_CLI_LIST_POOL_NODES = 2isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf1_cli_fsm_log_req {
    pub name: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf1_cli_fsm_log_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub op_errstr: String,
    pub fsm_log: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_get_volume {
    GF_CLI_GET_VOLUME_ALL = 1isize,
    GF_CLI_GET_VOLUME = 2isize,
    GF_CLI_GET_NEXT_VOLUME = 3isize,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gf1_cli_getwd_req {
    pub unused: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf1_cli_getwd_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub wd: String,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_gsync_set {
    GF_GSYNC_OPTION_TYPE_NONE = 0isize,
    GF_GSYNC_OPTION_TYPE_START = 1isize,
    GF_GSYNC_OPTION_TYPE_STOP = 2isize,
    GF_GSYNC_OPTION_TYPE_CONFIG = 3isize,
    GF_GSYNC_OPTION_TYPE_STATUS = 4isize,
    GF_GSYNC_OPTION_TYPE_ROTATE = 5isize,
    GF_GSYNC_OPTION_TYPE_CREATE = 6isize,
    GF_GSYNC_OPTION_TYPE_DELETE = 7isize,
    GF_GSYNC_OPTION_TYPE_PAUSE = 8isize,
    GF_GSYNC_OPTION_TYPE_RESUME = 9isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_info_op {
    GF_CLI_INFO_NONE = 0isize,
    GF_CLI_INFO_ALL = 1isize,
    GF_CLI_INFO_INCREMENTAL = 2isize,
    GF_CLI_INFO_CUMULATIVE = 3isize,
    GF_CLI_INFO_CLEAR = 4isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf1_cli_mount_req {
    pub label: String,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf1_cli_mount_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub path: String,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_op_flags { GF_CLI_FLAG_OP_FORCE = 1isize, }

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf1_cli_peer_list_req {
    pub flags: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf1_cli_peer_list_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub friends: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_snapshot {
    GF_SNAP_OPTION_TYPE_NONE = 0isize,
    GF_SNAP_OPTION_TYPE_CREATE = 1isize,
    GF_SNAP_OPTION_TYPE_DELETE = 2isize,
    GF_SNAP_OPTION_TYPE_RESTORE = 3isize,
    GF_SNAP_OPTION_TYPE_ACTIVATE = 4isize,
    GF_SNAP_OPTION_TYPE_DEACTIVATE = 5isize,
    GF_SNAP_OPTION_TYPE_LIST = 6isize,
    GF_SNAP_OPTION_TYPE_STATUS = 7isize,
    GF_SNAP_OPTION_TYPE_CONFIG = 8isize,
    GF_SNAP_OPTION_TYPE_CLONE = 9isize,
    GF_SNAP_OPTION_TYPE_INFO = 10isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_snapshot_config {
    GF_SNAP_CONFIG_TYPE_NONE = 0isize,
    GF_SNAP_CONFIG_TYPE_SET = 1isize,
    GF_SNAP_CONFIG_DISPLAY = 2isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_snapshot_delete {
    GF_SNAP_DELETE_TYPE_ALL = 0isize,
    GF_SNAP_DELETE_TYPE_SNAP = 1isize,
    GF_SNAP_DELETE_TYPE_VOL = 2isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_snapshot_info {
    GF_SNAP_INFO_TYPE_ALL = 0isize,
    GF_SNAP_INFO_TYPE_SNAP = 1isize,
    GF_SNAP_INFO_TYPE_VOL = 2isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_snapshot_status {
    GF_SNAP_STATUS_TYPE_ALL = 0isize,
    GF_SNAP_STATUS_TYPE_SNAP = 1isize,
    GF_SNAP_STATUS_TYPE_VOL = 2isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_stats_op {
    GF_CLI_STATS_NONE = 0isize,
    GF_CLI_STATS_START = 1isize,
    GF_CLI_STATS_STOP = 2isize,
    GF_CLI_STATS_INFO = 3isize,
    GF_CLI_STATS_TOP = 4isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_sync_volume { GF_CLI_SYNC_ALL = 1isize, }

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cli_top_op {
    GF_CLI_TOP_NONE = 0isize,
    GF_CLI_TOP_OPEN = 1isize,
    GF_CLI_TOP_READ = 2isize,
    GF_CLI_TOP_WRITE = 3isize,
    GF_CLI_TOP_OPENDIR = 4isize,
    GF_CLI_TOP_READDIR = 5isize,
    GF_CLI_TOP_READ_PERF = 6isize,
    GF_CLI_TOP_WRITE_PERF = 7isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf1_cli_umount_req {
    pub lazy: i32,
    pub path: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gf1_cli_umount_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_cluster_type {
    GF_CLUSTER_TYPE_NONE = 0isize,
    GF_CLUSTER_TYPE_STRIPE = 1isize,
    GF_CLUSTER_TYPE_REPLICATE = 2isize,
    GF_CLUSTER_TYPE_STRIPE_REPLICATE = 3isize,
    GF_CLUSTER_TYPE_DISPERSE = 4isize,
    GF_CLUSTER_TYPE_TIER = 5isize,
    GF_CLUSTER_TYPE_MAX = 6isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf1_op_commands {
    GF_OP_CMD_NONE = 0isize,
    GF_OP_CMD_START = 1isize,
    GF_OP_CMD_COMMIT = 2isize,
    GF_OP_CMD_STOP = 3isize,
    GF_OP_CMD_STATUS = 4isize,
    GF_OP_CMD_COMMIT_FORCE = 5isize,
    GF_OP_CMD_DETACH_START = 6isize,
    GF_OP_CMD_DETACH_COMMIT = 7isize,
    GF_OP_CMD_DETACH_COMMIT_FORCE = 8isize,
    GF_OP_CMD_STOP_DETACH_TIER = 9isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf_bitrot_type {
    GF_BITROT_OPTION_TYPE_NONE = 0isize,
    GF_BITROT_OPTION_TYPE_ENABLE = 1isize,
    GF_BITROT_OPTION_TYPE_DISABLE = 2isize,
    GF_BITROT_OPTION_TYPE_SCRUB_THROTTLE = 3isize,
    GF_BITROT_OPTION_TYPE_SCRUB_FREQ = 4isize,
    GF_BITROT_OPTION_TYPE_SCRUB = 5isize,
    GF_BITROT_OPTION_TYPE_EXPIRY_TIME = 6isize,
    GF_BITROT_OPTION_TYPE_MAX = 7isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf_cli_defrag_type {
    GF_DEFRAG_CMD_START = 1isize,
    GF_DEFRAG_CMD_STOP = 2isize,
    GF_DEFRAG_CMD_STATUS = 3isize,
    GF_DEFRAG_CMD_START_LAYOUT_FIX = 4isize,
    GF_DEFRAG_CMD_START_FORCE = 5isize,
    GF_DEFRAG_CMD_START_TIER = 6isize,
    GF_DEFRAG_CMD_STATUS_TIER = 7isize,
    GF_DEFRAG_CMD_START_DETACH_TIER = 8isize,
    GF_DEFRAG_CMD_STOP_DETACH_TIE = 9isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_cli_req {
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_cli_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub op_errstr: String,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf_cli_status_type {
    GF_CLI_STATUS_NONE = 0isize,
    GF_CLI_STATUS_MEM = 1isize,
    GF_CLI_STATUS_CLIENTS = 2isize,
    GF_CLI_STATUS_INODE = 4isize,
    GF_CLI_STATUS_FD = 8isize,
    GF_CLI_STATUS_CALLPOOL = 16isize,
    GF_CLI_STATUS_DETAIL = 32isize,
    GF_CLI_STATUS_TASKS = 64isize,
    GF_CLI_STATUS_MASK = 255isize,
    GF_CLI_STATUS_VOL = 256isize,
    GF_CLI_STATUS_ALL = 512isize,
    GF_CLI_STATUS_BRICK = 1024isize,
    GF_CLI_STATUS_NFS = 2048isize,
    GF_CLI_STATUS_SHD = 4096isize,
    GF_CLI_STATUS_QUOTAD = 8192isize,
    GF_CLI_STATUS_SNAPD = 16384isize,
    GF_CLI_STATUS_BITD = 32768isize,
    GF_CLI_STATUS_SCRUB = 65536isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_common_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf_defrag_status_t {
    GF_DEFRAG_STATUS_NOT_STARTED = 0isize,
    GF_DEFRAG_STATUS_STARTED = 1isize,
    GF_DEFRAG_STATUS_STOPPED = 2isize,
    GF_DEFRAG_STATUS_COMPLETE = 3isize,
    GF_DEFRAG_STATUS_FAILED = 4isize,
    GF_DEFRAG_STATUS_LAYOUT_FIX_STARTED = 5isize,
    GF_DEFRAG_STATUS_LAYOUT_FIX_STOPPED = 6isize,
    GF_DEFRAG_STATUS_LAYOUT_FIX_COMPLETE = 7isize,
    GF_DEFRAG_STATUS_LAYOUT_FIX_FAILED = 8isize,
    GF_DEFRAG_STATUS_MAX = 9isize,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gf_dump_req {
    pub gfs_id: i64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_dump_rsp {
    pub gfs_id: i64,
    pub op_ret: i32,
    pub op_errno: i32,
    pub prog: Option<Box<gf_prog_detail>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_event_notify_req {
    pub op: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_event_notify_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_get_volume_info_req {
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_get_volume_info_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub op_errstr: String,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_getsnap_name_uuid_req {
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_getsnap_name_uuid_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub op_errstr: String,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_getspec_req {
    pub flags: u32,
    pub key: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_getspec_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub spec: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gf_iatt {
    pub ia_gfid: [u8; 16i64 as usize],
    pub ia_ino: u_quad_t,
    pub ia_dev: u_quad_t,
    pub mode: u32,
    pub ia_nlink: u32,
    pub ia_uid: u32,
    pub ia_gid: u32,
    pub ia_rdev: u_quad_t,
    pub ia_size: u_quad_t,
    pub ia_blksize: u32,
    pub ia_blocks: u_quad_t,
    pub ia_atime: u32,
    pub ia_atime_nsec: u32,
    pub ia_mtime: u32,
    pub ia_mtime_nsec: u32,
    pub ia_ctime: u32,
    pub ia_ctime_nsec: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_log_req {
    pub msg: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_mgmt_hndsk_req {
    pub hndsk: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_mgmt_hndsk_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub hndsk: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_notify_req {
    pub flags: u32,
    pub buf: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_notify_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub flags: u32,
    pub buf: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_prog_detail {
    pub progname: String,
    pub prognum: i64,
    pub progver: i64,
    pub next: Option<Box<gf_prog_detail>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_proto_flock {
    pub flock_type: u32,
    pub whence: u32,
    pub start: u_quad_t,
    pub len: u_quad_t,
    pub pid: u32,
    pub lk_owner: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum gf_quota_type {
    GF_QUOTA_OPTION_TYPE_NONE = 0isize,
    GF_QUOTA_OPTION_TYPE_ENABLE = 1isize,
    GF_QUOTA_OPTION_TYPE_DISABLE = 2isize,
    GF_QUOTA_OPTION_TYPE_LIMIT_USAGE = 3isize,
    GF_QUOTA_OPTION_TYPE_REMOVE = 4isize,
    GF_QUOTA_OPTION_TYPE_LIST = 5isize,
    GF_QUOTA_OPTION_TYPE_VERSION = 6isize,
    GF_QUOTA_OPTION_TYPE_ALERT_TIME = 7isize,
    GF_QUOTA_OPTION_TYPE_SOFT_TIMEOUT = 8isize,
    GF_QUOTA_OPTION_TYPE_HARD_TIMEOUT = 9isize,
    GF_QUOTA_OPTION_TYPE_DEFAULT_SOFT_LIMIT = 10isize,
    GF_QUOTA_OPTION_TYPE_VERSION_OBJECTS = 11isize,
    GF_QUOTA_OPTION_TYPE_LIMIT_OBJECTS = 12isize,
    GF_QUOTA_OPTION_TYPE_LIST_OBJECTS = 13isize,
    GF_QUOTA_OPTION_TYPE_REMOVE_OBJECTS = 14isize,
    GF_QUOTA_OPTION_TYPE_ENABLE_OBJECTS = 15isize,
    GF_QUOTA_OPTION_TYPE_MAX = 16isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_set_lk_ver_req {
    pub uid: String,
    pub lk_ver: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gf_set_lk_ver_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub lk_ver: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_setvolume_req {
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gf_setvolume_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct gf_statfs {
    pub bsize: u_quad_t,
    pub frsize: u_quad_t,
    pub blocks: u_quad_t,
    pub bfree: u_quad_t,
    pub bavail: u_quad_t,
    pub files: u_quad_t,
    pub ffree: u_quad_t,
    pub favail: u_quad_t,
    pub fsid: u_quad_t,
    pub flag: u_quad_t,
    pub namemax: u_quad_t,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_access_req {
    pub gfid: [u8; 16i64 as usize],
    pub mask: u32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_cbk_cache_invalidation_req {
    pub gfid: String,
    pub event_type: u32,
    pub flags: u32,
    pub expire_time_attr: u32,
    pub stat: gf_iatt,
    pub parent_stat: gf_iatt,
    pub oldparent_stat: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_create_req {
    pub pargfid: [u8; 16i64 as usize],
    pub flags: u32,
    pub mode: u32,
    pub umask: u32,
    pub bname: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_create_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub fd: u_quad_t,
    pub preparent: gf_iatt,
    pub postparent: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_dirlist {
    pub d_ino: u_quad_t,
    pub d_off: u_quad_t,
    pub d_len: u32,
    pub d_type: u32,
    pub name: String,
    pub nextentry: Option<Box<gfs3_dirlist>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_dirplist {
    pub d_ino: u_quad_t,
    pub d_off: u_quad_t,
    pub d_len: u32,
    pub d_type: u32,
    pub name: String,
    pub stat: gf_iatt,
    pub dict: Vec<u8>,
    pub nextentry: Option<Box<gfs3_dirplist>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_discard_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub offset: u_quad_t,
    pub size: u_quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_discard_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub statpre: gf_iatt,
    pub statpost: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_entrylk_req {
    pub gfid: [u8; 16i64 as usize],
    pub cmd: u32,
    pub entrylk_type: u32,
    pub namelen: u_quad_t,
    pub name: String,
    pub volume: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fallocate_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub flags: u32,
    pub offset: u_quad_t,
    pub size: u_quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fallocate_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub statpre: gf_iatt,
    pub statpost: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fentrylk_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub cmd: u32,
    pub fentrylk_type: u32,
    pub namelen: u_quad_t,
    pub name: String,
    pub volume: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fgetxattr_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub namelen: u32,
    pub name: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fgetxattr_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub dict: Vec<u8>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_finodelk_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub cmd: u32,
    pub finodelk_type: u32,
    pub flock: gf_proto_flock,
    pub volume: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fremovexattr_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub name: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fsetattr_req {
    pub fd: quad_t,
    pub stbuf: gf_iatt,
    pub valid: i32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fsetattr_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub statpre: gf_iatt,
    pub statpost: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fsetxattr_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: int64_t,
    pub flags: u32,
    pub dict: Vec<u8>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fstat_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fstat_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fsync_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub prestat: gf_iatt,
    pub poststat: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fsyncdir_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub data: i32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_ftruncate_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub offset: u_quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_ftruncate_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub prestat: gf_iatt,
    pub poststat: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fxattrop_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub flags: u32,
    pub dict: Vec<u8>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_fxattrop_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub dict: Vec<u8>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_getxattr_req {
    pub gfid: [u8; 16i64 as usize],
    pub namelen: u32,
    pub name: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_getxattr_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub dict: Vec<u8>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_inodelk_req {
    pub gfid: [u8; 16i64 as usize],
    pub cmd: u32,
    pub inodelk_type: u32,
    pub flock: gf_proto_flock,
    pub volume: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_ipc_req {
    pub op: i32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_ipc_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_link_req {
    pub oldgfid: [u8; 16i64 as usize],
    pub newgfid: [u8; 16i64 as usize],
    pub newbname: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_link_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub preparent: gf_iatt,
    pub postparent: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_lk_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: int64_t,
    pub cmd: u32,
    pub lk_type: u32,
    pub flock: gf_proto_flock,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_lk_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub flock: gf_proto_flock,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_lookup_req {
    pub gfid: [u8; 16i64 as usize],
    pub pargfid: [u8; 16i64 as usize],
    pub flags: u32,
    pub bname: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_lookup_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub postparent: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_mkdir_req {
    pub pargfid: [u8; 16i64 as usize],
    pub mode: u32,
    pub umask: u32,
    pub bname: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_mkdir_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub preparent: gf_iatt,
    pub postparent: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_mknod_req {
    pub pargfid: [u8; 16i64 as usize],
    pub dev: u_quad_t,
    pub mode: u32,
    pub umask: u32,
    pub bname: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_mknod_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub preparent: gf_iatt,
    pub postparent: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_open_req {
    pub gfid: [u8; 16i64 as usize],
    pub flags: u32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_open_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub fd: quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_opendir_req {
    pub gfid: [u8; 16i64 as usize],
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_opendir_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub fd: quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_rchecksum_req {
    pub fd: quad_t,
    pub offset: u_quad_t,
    pub len: u32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_rchecksum_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub weak_checksum: u32,
    pub strong_checksum: Vec<u8>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_read_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub offset: u_quad_t,
    pub size: u32,
    pub flag: u32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_read_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub size: u32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_readdir_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub offset: u_quad_t,
    pub size: u32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_readdir_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub reply: Option<Box<gfs3_dirlist>>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_readdirp_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub offset: u_quad_t,
    pub size: u32,
    pub dict: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_readdirp_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub reply: Option<Box<gfs3_dirplist>>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_readlink_req {
    pub gfid: [u8; 16i64 as usize],
    pub size: u32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_readlink_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub buf: gf_iatt,
    pub path: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_release_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_releasedir_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_removexattr_req {
    pub gfid: [u8; 16i64 as usize],
    pub name: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_rename_req {
    pub oldgfid: [u8; 16i64 as usize],
    pub newgfid: [u8; 16i64 as usize],
    pub oldbname: String,
    pub newbname: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_rename_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub preoldparent: gf_iatt,
    pub postoldparent: gf_iatt,
    pub prenewparent: gf_iatt,
    pub postnewparent: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_rmdir_req {
    pub pargfid: [u8; 16i64 as usize],
    pub xflags: i32,
    pub bname: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_rmdir_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub preparent: gf_iatt,
    pub postparent: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_setattr_req {
    pub gfid: [u8; 16i64 as usize],
    pub stbuf: gf_iatt,
    pub valid: i32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_setattr_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub statpre: gf_iatt,
    pub statpost: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_setxattr_req {
    pub gfid: [u8; 16i64 as usize],
    pub flags: u32,
    pub dict: Vec<u8>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_stat_req {
    pub gfid: [u8; 16i64 as usize],
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_stat_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_statfs_req {
    pub gfid: [u8; 16i64 as usize],
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_statfs_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub statfs: gf_statfs,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_symlink_req {
    pub pargfid: [u8; 16i64 as usize],
    pub bname: String,
    pub umask: u32,
    pub linkname: String,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_symlink_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub stat: gf_iatt,
    pub preparent: gf_iatt,
    pub postparent: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_truncate_req {
    pub gfid: [u8; 16i64 as usize],
    pub offset: u_quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_truncate_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub prestat: gf_iatt,
    pub poststat: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_unlink_req {
    pub pargfid: [u8; 16i64 as usize],
    pub bname: String,
    pub xflags: u32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_unlink_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub preparent: gf_iatt,
    pub postparent: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_write_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub offset: u_quad_t,
    pub size: u32,
    pub flag: u32,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_write_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub prestat: gf_iatt,
    pub poststat: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_xattrop_req {
    pub gfid: [u8; 16i64 as usize],
    pub flags: u32,
    pub dict: Vec<u8>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_xattrop_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub dict: Vec<u8>,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_zerofill_req {
    pub gfid: [u8; 16i64 as usize],
    pub fd: quad_t,
    pub offset: u_quad_t,
    pub size: u_quad_t,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct gfs3_zerofill_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub statpre: gf_iatt,
    pub statpost: gf_iatt,
    pub xdata: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum glusterd_volume_status {
    GLUSTERD_STATUS_NONE = 0isize,
    GLUSTERD_STATUS_STARTED = 1isize,
    GLUSTERD_STATUS_STOPPED = 2isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct mon {
    pub mon_id: mon_id,
    pub private: [u8; 16i64 as usize],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct mon_id {
    pub mon_name: String,
    pub my_id: my_id,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct my_id {
    pub my_name: String,
    pub my_prog: i32,
    pub my_vers: i32,
    pub my_proc: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct nlm4_freeallargs {
    pub name: String,
    pub state: uint32_t,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct nlm4_stat {
    pub stat: nlm4_stats,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum nlm4_stats {
    nlm4_granted = 0isize,
    nlm4_denied = 1isize,
    nlm4_denied_nolock = 2isize,
    nlm4_blocked = 3isize,
    nlm4_denied_grace_period = 4isize,
    nlm4_deadlck = 5isize,
    nlm4_rofs = 6isize,
    nlm4_stale_fh = 7isize,
    nlm4_fbig = 8isize,
    nlm4_failed = 9isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct nlm_sm_status {
    pub mon_name: String,
    pub state: i32,
    pub private: [u8; 16i64 as usize],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct nsm_callback_status {
    pub mon_name: String,
    pub state: i32,
    pub private: [u8; 16i64 as usize],
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct pmap_brick_by_port_req {
    pub port: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct pmap_brick_by_port_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub status: i32,
    pub brick: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct pmap_port_by_brick_req {
    pub brick: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct pmap_port_by_brick_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
    pub status: i32,
    pub port: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct pmap_signin_req {
    pub brick: String,
    pub port: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct pmap_signin_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct pmap_signout_req {
    pub brick: String,
    pub port: i32,
    pub rdma_port: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct pmap_signout_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct pmap_signup_req {
    pub brick: String,
    pub port: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct pmap_signup_rsp {
    pub op_ret: i32,
    pub op_errno: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum res { STAT_SUCC = 0isize, STAT_FAIL = 1isize, }

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct sm_name {
    pub mon_name: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct sm_stat {
    pub state: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct sm_stat_res {
    pub res_stat: res,
    pub state: i32,
}

pub type int32_t = i32;

pub type int64_t = i64;

pub type mntudpdirpath = String;

pub type quad_t = i64;

pub type u_int32_t = u32;

pub type u_int64_t = u64;

pub type u_quad_t = u64;

pub type uint32_t = u32;

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for aclentry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . acl_type . pack ( out )) +
               try!(self . uid . pack ( out )) +
               try!(self . perm . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for auth_glusterfs_parms {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . lk_owner . pack ( out )) +
               try!(self . pid . pack ( out )) +
               try!(self . uid . pack ( out )) +
               try!(self . gid . pack ( out )) +
               try!(self . ngrps . pack ( out )) +
               try!(xdr_codec :: pack_array (
                    & self . groups [ .. ] , self . groups . len (  ) , out ,
                    None )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for auth_glusterfs_parms_v2
 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . pid . pack ( out )) + try!(self . uid . pack ( out )) +
               try!(self . gid . pack ( out )) +
               try!(xdr_codec :: pack_flex ( & self . groups , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . lk_owner , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for changelog_event_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . seq . pack ( out )) +
               try!(self . tv_sec . pack ( out )) +
               try!(self . tv_usec . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for changelog_event_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . seq . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for changelog_probe_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for fsh_access {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for fsh_mode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_brick_op_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . name , None , out )) +
               try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . input , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_brick_op_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . output , None , out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for
 gd1_mgmt_cluster_lock_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for
 gd1_mgmt_cluster_lock_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for
 gd1_mgmt_cluster_unlock_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for
 gd1_mgmt_cluster_unlock_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_commit_op_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . buf , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_commit_op_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_friend_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_string ( & self . hostname , None , out
                    )) + try!(self . port . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . vols , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_friend_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_string ( & self . hostname , None , out
                    )) + try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . port . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_friend_update {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . friends , None , out )) +
               try!(self . port . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for
 gd1_mgmt_friend_update_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_probe_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_string ( & self . hostname , None , out
                    )) + try!(self . port . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_stage_op_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . buf , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_stage_op_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_unfriend_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_string ( & self . hostname , None , out
                    )) + try!(self . port . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_unfriend_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_string ( & self . hostname , None , out
                    )) + try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . port . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_brick_op_req
 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_brick_op_rsp
 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_commit_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_commit_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_lock_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . txn_id [ .. ] , self . txn_id . len (  ) , out ))
               + try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_lock_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . txn_id [ .. ] , self . txn_id . len (  ) , out ))
               +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_post_val_req
 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_post_val_rsp
 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_pre_val_req
 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_pre_val_rsp
 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(self . op . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_unlock_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . txn_id [ .. ] , self . txn_id . len (  ) , out ))
               + try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gd1_mgmt_v3_unlock_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . uuid [ .. ] , self . uuid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . txn_id [ .. ] , self . txn_id . len (  ) , out ))
               +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_friends_list {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_fsm_log_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . name , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_fsm_log_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . fsm_log , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_get_volume {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_getwd_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . unused . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_getwd_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . wd , None , out )) +
               0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_gsync_set {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_info_op {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_mount_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . label , None , out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_mount_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . path , None , out )) +
               0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_op_flags {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_peer_list_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_peer_list_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . friends , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_snapshot {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_snapshot_config
 {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_snapshot_delete
 {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_snapshot_info {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_snapshot_status
 {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_stats_op {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_sync_volume {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_top_op {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_umount_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . lazy . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . path , None , out )) +
               0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cli_umount_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_cluster_type {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf1_op_commands {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_bitrot_type {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_cli_defrag_type {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_cli_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out ))
               + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_cli_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_cli_status_type {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_common_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_defrag_status_t {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_dump_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . gfs_id . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_dump_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . gfs_id . pack ( out )) +
               try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . prog . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_event_notify_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_event_notify_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_get_volume_info_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out ))
               + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_get_volume_info_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_getsnap_name_uuid_req
 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out ))
               + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_getsnap_name_uuid_rsp
 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . op_errstr , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_getspec_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . key , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_getspec_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . spec , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_iatt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . ia_gfid [ .. ] , self . ia_gfid . len (  ) , out )) +
               try!(self . ia_ino . pack ( out )) +
               try!(self . ia_dev . pack ( out )) +
               try!(self . mode . pack ( out )) +
               try!(self . ia_nlink . pack ( out )) +
               try!(self . ia_uid . pack ( out )) +
               try!(self . ia_gid . pack ( out )) +
               try!(self . ia_rdev . pack ( out )) +
               try!(self . ia_size . pack ( out )) +
               try!(self . ia_blksize . pack ( out )) +
               try!(self . ia_blocks . pack ( out )) +
               try!(self . ia_atime . pack ( out )) +
               try!(self . ia_atime_nsec . pack ( out )) +
               try!(self . ia_mtime . pack ( out )) +
               try!(self . ia_mtime_nsec . pack ( out )) +
               try!(self . ia_ctime . pack ( out )) +
               try!(self . ia_ctime_nsec . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_log_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_flex ( & self . msg , None , out )) +
               0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_mgmt_hndsk_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_flex ( & self . hndsk , None , out ))
               + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_mgmt_hndsk_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . hndsk , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_notify_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . buf , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_notify_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . buf , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_prog_detail {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . progname , None , out )) +
               try!(self . prognum . pack ( out )) +
               try!(self . progver . pack ( out )) +
               try!(self . next . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_proto_flock {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . flock_type . pack ( out )) +
               try!(self . whence . pack ( out )) +
               try!(self . start . pack ( out )) +
               try!(self . len . pack ( out )) +
               try!(self . pid . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . lk_owner , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_quota_type {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_set_lk_ver_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . uid , None , out )) +
               try!(self . lk_ver . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_set_lk_ver_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . lk_ver . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_setvolume_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out ))
               + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_setvolume_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gf_statfs {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . bsize . pack ( out )) +
               try!(self . frsize . pack ( out )) +
               try!(self . blocks . pack ( out )) +
               try!(self . bfree . pack ( out )) +
               try!(self . bavail . pack ( out )) +
               try!(self . files . pack ( out )) +
               try!(self . ffree . pack ( out )) +
               try!(self . favail . pack ( out )) +
               try!(self . fsid . pack ( out )) +
               try!(self . flag . pack ( out )) +
               try!(self . namemax . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_access_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . mask . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for
 gfs3_cbk_cache_invalidation_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . gfid , None , out )) +
               try!(self . event_type . pack ( out )) +
               try!(self . flags . pack ( out )) +
               try!(self . expire_time_attr . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(self . parent_stat . pack ( out )) +
               try!(self . oldparent_stat . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_create_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . pargfid [ .. ] , self . pargfid . len (  ) , out )) +
               try!(self . flags . pack ( out )) +
               try!(self . mode . pack ( out )) +
               try!(self . umask . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . bname , None , out ))
               +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_create_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(self . fd . pack ( out )) +
               try!(self . preparent . pack ( out )) +
               try!(self . postparent . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_dirlist {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . d_ino . pack ( out )) +
               try!(self . d_off . pack ( out )) +
               try!(self . d_len . pack ( out )) +
               try!(self . d_type . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . name , None , out )) +
               try!(self . nextentry . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_dirplist {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . d_ino . pack ( out )) +
               try!(self . d_off . pack ( out )) +
               try!(self . d_len . pack ( out )) +
               try!(self . d_type . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . name , None , out )) +
               try!(self . stat . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + try!(self . nextentry . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_discard_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . offset . pack ( out )) +
               try!(self . size . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_discard_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . statpre . pack ( out )) +
               try!(self . statpost . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_entrylk_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . cmd . pack ( out )) +
               try!(self . entrylk_type . pack ( out )) +
               try!(self . namelen . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . name , None , out )) +
               try!(xdr_codec :: pack_string ( & self . volume , None , out ))
               +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fallocate_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . flags . pack ( out )) +
               try!(self . offset . pack ( out )) +
               try!(self . size . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fallocate_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . statpre . pack ( out )) +
               try!(self . statpost . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fentrylk_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . cmd . pack ( out )) +
               try!(self . fentrylk_type . pack ( out )) +
               try!(self . namelen . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . name , None , out )) +
               try!(xdr_codec :: pack_string ( & self . volume , None , out ))
               +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fgetxattr_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . namelen . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . name , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fgetxattr_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_finodelk_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . cmd . pack ( out )) +
               try!(self . finodelk_type . pack ( out )) +
               try!(self . flock . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . volume , None , out ))
               +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fremovexattr_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . name , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fsetattr_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . fd . pack ( out )) + try!(self . stbuf . pack ( out ))
               + try!(self . valid . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fsetattr_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . statpre . pack ( out )) +
               try!(self . statpost . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fsetxattr_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fstat_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fstat_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fsync_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . prestat . pack ( out )) +
               try!(self . poststat . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fsyncdir_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . data . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_ftruncate_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . offset . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_ftruncate_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . prestat . pack ( out )) +
               try!(self . poststat . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fxattrop_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_fxattrop_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_getxattr_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . namelen . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . name , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_getxattr_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_inodelk_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . cmd . pack ( out )) +
               try!(self . inodelk_type . pack ( out )) +
               try!(self . flock . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . volume , None , out ))
               +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_ipc_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_ipc_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_link_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . oldgfid [ .. ] , self . oldgfid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . newgfid [ .. ] , self . newgfid . len (  ) , out
                    )) +
               try!(xdr_codec :: pack_string ( & self . newbname , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_link_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(self . preparent . pack ( out )) +
               try!(self . postparent . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_lk_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . cmd . pack ( out )) +
               try!(self . lk_type . pack ( out )) +
               try!(self . flock . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_lk_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . flock . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_lookup_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . pargfid [ .. ] , self . pargfid . len (  ) , out
                    )) + try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . bname , None , out ))
               +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_lookup_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(self . postparent . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_mkdir_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . pargfid [ .. ] , self . pargfid . len (  ) , out )) +
               try!(self . mode . pack ( out )) +
               try!(self . umask . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . bname , None , out ))
               +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_mkdir_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(self . preparent . pack ( out )) +
               try!(self . postparent . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_mknod_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . pargfid [ .. ] , self . pargfid . len (  ) , out )) +
               try!(self . dev . pack ( out )) +
               try!(self . mode . pack ( out )) +
               try!(self . umask . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . bname , None , out ))
               +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_mknod_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(self . preparent . pack ( out )) +
               try!(self . postparent . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_open_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_open_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . fd . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_opendir_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_opendir_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . fd . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_rchecksum_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . fd . pack ( out )) + try!(self . offset . pack ( out ))
               + try!(self . len . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_rchecksum_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . weak_checksum . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . strong_checksum , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_read_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . offset . pack ( out )) +
               try!(self . size . pack ( out )) +
               try!(self . flag . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_read_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(self . size . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_readdir_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . offset . pack ( out )) +
               try!(self . size . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_readdir_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . reply . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_readdirp_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . offset . pack ( out )) +
               try!(self . size . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_readdirp_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . reply . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_readlink_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . size . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_readlink_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . buf . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . path , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_release_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_releasedir_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_removexattr_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(xdr_codec :: pack_string ( & self . name , None , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_rename_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . oldgfid [ .. ] , self . oldgfid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . newgfid [ .. ] , self . newgfid . len (  ) , out
                    )) +
               try!(xdr_codec :: pack_string ( & self . oldbname , None , out
                    )) +
               try!(xdr_codec :: pack_string ( & self . newbname , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_rename_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(self . preoldparent . pack ( out )) +
               try!(self . postoldparent . pack ( out )) +
               try!(self . prenewparent . pack ( out )) +
               try!(self . postnewparent . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_rmdir_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . pargfid [ .. ] , self . pargfid . len (  ) , out )) +
               try!(self . xflags . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . bname , None , out ))
               +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_rmdir_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . preparent . pack ( out )) +
               try!(self . postparent . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_setattr_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . stbuf . pack ( out )) +
               try!(self . valid . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_setattr_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . statpre . pack ( out )) +
               try!(self . statpost . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_setxattr_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_stat_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_stat_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_statfs_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_statfs_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . statfs . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_symlink_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . pargfid [ .. ] , self . pargfid . len (  ) , out )) +
               try!(xdr_codec :: pack_string ( & self . bname , None , out ))
               + try!(self . umask . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . linkname , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_symlink_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . stat . pack ( out )) +
               try!(self . preparent . pack ( out )) +
               try!(self . postparent . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_truncate_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . offset . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_truncate_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . prestat . pack ( out )) +
               try!(self . poststat . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_unlink_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . pargfid [ .. ] , self . pargfid . len (  ) , out )) +
               try!(xdr_codec :: pack_string ( & self . bname , None , out ))
               + try!(self . xflags . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_unlink_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . preparent . pack ( out )) +
               try!(self . postparent . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_write_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . offset . pack ( out )) +
               try!(self . size . pack ( out )) +
               try!(self . flag . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_write_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . prestat . pack ( out )) +
               try!(self . poststat . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_xattrop_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . flags . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_xattrop_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex ( & self . dict , None , out
                    )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_zerofill_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_opaque_array (
                & self . gfid [ .. ] , self . gfid . len (  ) , out )) +
               try!(self . fd . pack ( out )) +
               try!(self . offset . pack ( out )) +
               try!(self . size . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for gfs3_zerofill_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . statpre . pack ( out )) +
               try!(self . statpost . pack ( out )) +
               try!(xdr_codec :: pack_opaque_flex (
                    & self . xdata , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for glusterd_volume_status {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for mon {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . mon_id . pack ( out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . private [ .. ] , self . private . len (  ) , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for mon_id {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string (
                & self . mon_name , Some ( SM_MAXSTRLEN as usize ) , out )) +
               try!(self . my_id . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for my_id {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string (
                & self . my_name , Some ( SM_MAXSTRLEN as usize ) , out )) +
               try!(self . my_prog . pack ( out )) +
               try!(self . my_vers . pack ( out )) +
               try!(self . my_proc . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for nlm4_freeallargs {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string (
                & self . name , Some ( LM_MAXSTRLEN as usize ) , out )) +
               try!(self . state . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for nlm4_stat {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . stat . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for nlm4_stats {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for nlm_sm_status {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string (
                & self . mon_name , Some ( LM_MAXSTRLEN as usize ) , out )) +
               try!(self . state . pack ( out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . private [ .. ] , self . private . len (  ) , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for nsm_callback_status {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string (
                & self . mon_name , Some ( SM_MAXSTRLEN as usize ) , out )) +
               try!(self . state . pack ( out )) +
               try!(xdr_codec :: pack_opaque_array (
                    & self . private [ .. ] , self . private . len (  ) , out
                    )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_brick_by_port_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . port . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_brick_by_port_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . status . pack ( out )) +
               try!(xdr_codec :: pack_string ( & self . brick , None , out ))
               + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_port_by_brick_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . brick , None , out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_port_by_brick_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) +
               try!(self . status . pack ( out )) +
               try!(self . port . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_signin_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . brick , None , out )) +
               try!(self . port . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_signin_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_signout_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . brick , None , out )) +
               try!(self . port . pack ( out )) +
               try!(self . rdma_port . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_signout_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_signup_req {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string ( & self . brick , None , out )) +
               try!(self . port . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for pmap_signup_rsp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . op_ret . pack ( out )) +
               try!(self . op_errno . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for res {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(( * self as i32 ) . pack ( out )))
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for sm_name {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(xdr_codec :: pack_string (
                & self . mon_name , Some ( SM_MAXSTRLEN as usize ) , out )) +
               0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for sm_stat {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . state . pack ( out )) + 0)
    }
}

impl <Out: xdr_codec::Write> xdr_codec::Pack<Out> for sm_stat_res {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(try!(self . res_stat . pack ( out )) +
               try!(self . state . pack ( out )) + 0)
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for aclentry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(aclentry, usize)> {
        let mut sz = 0;
        Ok((aclentry{acl_type:
                         {
                             let (v, fsz) =
                                 try!(xdr_codec :: Unpack :: unpack ( input
                                      ));
                             sz += fsz;
                             v
                         },
                     uid:
                         {
                             let (v, fsz) =
                                 try!(xdr_codec :: Unpack :: unpack ( input
                                      ));
                             sz += fsz;
                             v
                         },
                     perm:
                         {
                             let (v, fsz) =
                                 try!(xdr_codec :: Unpack :: unpack ( input
                                      ));
                             sz += fsz;
                             v
                         },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for auth_glusterfs_parms {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(auth_glusterfs_parms, usize)> {
        let mut sz = 0;
        Ok((auth_glusterfs_parms{lk_owner:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec :: Unpack ::
                                                  unpack ( input ));
                                         sz += fsz;
                                         v
                                     },
                                 pid:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec :: Unpack ::
                                                  unpack ( input ));
                                         sz += fsz;
                                         v
                                     },
                                 uid:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec :: Unpack ::
                                                  unpack ( input ));
                                         sz += fsz;
                                         v
                                     },
                                 gid:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec :: Unpack ::
                                                  unpack ( input ));
                                         sz += fsz;
                                         v
                                     },
                                 ngrps:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec :: Unpack ::
                                                  unpack ( input ));
                                         sz += fsz;
                                         v
                                     },
                                 groups:
                                     {
                                         let (v, fsz) =
                                             {
                                                 use std::mem;
                                                 let mut buf:
                                                         [u32; 16i64 as
                                                                   usize] =
                                                     unsafe {
                                                         mem::uninitialized()
                                                     };
                                                 let sz =
                                                     try!(xdr_codec ::
                                                          unpack_array (
                                                          input , & mut buf [
                                                          .. ] , 16i64 as
                                                          usize , None ));
                                                 (buf, sz)
                                             };
                                         sz += fsz;
                                         v
                                     },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for auth_glusterfs_parms_v2 {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(auth_glusterfs_parms_v2, usize)> {
        let mut sz = 0;
        Ok((auth_glusterfs_parms_v2{pid:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec :: Unpack ::
                                                     unpack ( input ));
                                            sz += fsz;
                                            v
                                        },
                                    uid:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec :: Unpack ::
                                                     unpack ( input ));
                                            sz += fsz;
                                            v
                                        },
                                    gid:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec :: Unpack ::
                                                     unpack ( input ));
                                            sz += fsz;
                                            v
                                        },
                                    groups:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec :: unpack_flex
                                                     ( input , None ));
                                            sz += fsz;
                                            v
                                        },
                                    lk_owner:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec ::
                                                     unpack_opaque_flex (
                                                     input , None ));
                                            sz += fsz;
                                            v
                                        },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for changelog_event_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(changelog_event_req, usize)> {
        let mut sz = 0;
        Ok((changelog_event_req{seq:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                tv_sec:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                tv_usec:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for changelog_event_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(changelog_event_rsp, usize)> {
        let mut sz = 0;
        Ok((changelog_event_rsp{op_ret:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                seq:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for changelog_probe_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(changelog_probe_rsp, usize)> {
        let mut sz = 0;
        Ok((changelog_probe_rsp{op_ret:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for fsh_access {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(fsh_access, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (fsh_access::fsa_NONE as i32) =>
                    fsh_access::fsa_NONE,
                    x if x == (fsh_access::fsa_R as i32) => fsh_access::fsa_R,
                    x if x == (fsh_access::fsa_W as i32) => fsh_access::fsa_W,
                    x if x == (fsh_access::fsa_RW as i32) =>
                    fsh_access::fsa_RW,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for fsh_mode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(fsh_mode, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (fsh_mode::fsm_DN as i32) => fsh_mode::fsm_DN,
                    x if x == (fsh_mode::fsm_DR as i32) => fsh_mode::fsm_DR,
                    x if x == (fsh_mode::fsm_DW as i32) => fsh_mode::fsm_DW,
                    x if x == (fsh_mode::fsm_DRW as i32) => fsh_mode::fsm_DRW,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_brick_op_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_brick_op_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_brick_op_req{name:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: unpack_string
                                                   ( input , None ));
                                          sz += fsz;
                                          v
                                      },
                                  op:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  input:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec ::
                                                   unpack_opaque_flex (
                                                   input , None ));
                                          sz += fsz;
                                          v
                                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_brick_op_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_brick_op_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_brick_op_rsp{op_ret:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  op_errno:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  output:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec ::
                                                   unpack_opaque_flex (
                                                   input , None ));
                                          sz += fsz;
                                          v
                                      },
                                  op_errstr:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: unpack_string
                                                   ( input , None ));
                                          sz += fsz;
                                          v
                                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_cluster_lock_req
 {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_cluster_lock_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_cluster_lock_req{uuid:
                                          {
                                              let (v, fsz) =
                                                  {
                                                      use std::mem;
                                                      let mut buf:
                                                              [u8; 16i64 as
                                                                       usize] =
                                                          unsafe {
                                                              mem::uninitialized()
                                                          };
                                                      let sz =
                                                          try!(xdr_codec ::
                                                               unpack_opaque_array
                                                               (
                                                               input , & mut
                                                               buf [ .. ] ,
                                                               16i64 as usize
                                                               ));
                                                      (buf, sz)
                                                  };
                                              sz += fsz;
                                              v
                                          },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_cluster_lock_rsp
 {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_cluster_lock_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_cluster_lock_rsp{uuid:
                                          {
                                              let (v, fsz) =
                                                  {
                                                      use std::mem;
                                                      let mut buf:
                                                              [u8; 16i64 as
                                                                       usize] =
                                                          unsafe {
                                                              mem::uninitialized()
                                                          };
                                                      let sz =
                                                          try!(xdr_codec ::
                                                               unpack_opaque_array
                                                               (
                                                               input , & mut
                                                               buf [ .. ] ,
                                                               16i64 as usize
                                                               ));
                                                      (buf, sz)
                                                  };
                                              sz += fsz;
                                              v
                                          },
                                      op_ret:
                                          {
                                              let (v, fsz) =
                                                  try!(xdr_codec :: Unpack ::
                                                       unpack ( input ));
                                              sz += fsz;
                                              v
                                          },
                                      op_errno:
                                          {
                                              let (v, fsz) =
                                                  try!(xdr_codec :: Unpack ::
                                                       unpack ( input ));
                                              sz += fsz;
                                              v
                                          },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for
 gd1_mgmt_cluster_unlock_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_cluster_unlock_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_cluster_unlock_req{uuid:
                                            {
                                                let (v, fsz) =
                                                    {
                                                        use std::mem;
                                                        let mut buf:
                                                                [u8; 16i64 as
                                                                         usize] =
                                                            unsafe {
                                                                mem::uninitialized()
                                                            };
                                                        let sz =
                                                            try!(xdr_codec ::
                                                                 unpack_opaque_array
                                                                 (
                                                                 input , & mut
                                                                 buf [ .. ] ,
                                                                 16i64 as
                                                                 usize ));
                                                        (buf, sz)
                                                    };
                                                sz += fsz;
                                                v
                                            },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for
 gd1_mgmt_cluster_unlock_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_cluster_unlock_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_cluster_unlock_rsp{uuid:
                                            {
                                                let (v, fsz) =
                                                    {
                                                        use std::mem;
                                                        let mut buf:
                                                                [u8; 16i64 as
                                                                         usize] =
                                                            unsafe {
                                                                mem::uninitialized()
                                                            };
                                                        let sz =
                                                            try!(xdr_codec ::
                                                                 unpack_opaque_array
                                                                 (
                                                                 input , & mut
                                                                 buf [ .. ] ,
                                                                 16i64 as
                                                                 usize ));
                                                        (buf, sz)
                                                    };
                                                sz += fsz;
                                                v
                                            },
                                        op_ret:
                                            {
                                                let (v, fsz) =
                                                    try!(xdr_codec :: Unpack
                                                         :: unpack ( input ));
                                                sz += fsz;
                                                v
                                            },
                                        op_errno:
                                            {
                                                let (v, fsz) =
                                                    try!(xdr_codec :: Unpack
                                                         :: unpack ( input ));
                                                sz += fsz;
                                                v
                                            },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_commit_op_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_commit_op_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_commit_op_req{uuid:
                                       {
                                           let (v, fsz) =
                                               {
                                                   use std::mem;
                                                   let mut buf:
                                                           [u8; 16i64 as
                                                                    usize] =
                                                       unsafe {
                                                           mem::uninitialized()
                                                       };
                                                   let sz =
                                                       try!(xdr_codec ::
                                                            unpack_opaque_array
                                                            (
                                                            input , & mut buf
                                                            [ .. ] , 16i64 as
                                                            usize ));
                                                   (buf, sz)
                                               };
                                           sz += fsz;
                                           v
                                       },
                                   op:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   buf:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec ::
                                                    unpack_opaque_flex (
                                                    input , None ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_commit_op_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_commit_op_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_commit_op_rsp{uuid:
                                       {
                                           let (v, fsz) =
                                               {
                                                   use std::mem;
                                                   let mut buf:
                                                           [u8; 16i64 as
                                                                    usize] =
                                                       unsafe {
                                                           mem::uninitialized()
                                                       };
                                                   let sz =
                                                       try!(xdr_codec ::
                                                            unpack_opaque_array
                                                            (
                                                            input , & mut buf
                                                            [ .. ] , 16i64 as
                                                            usize ));
                                                   (buf, sz)
                                               };
                                           sz += fsz;
                                           v
                                       },
                                   op:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   op_ret:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   op_errno:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   dict:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec ::
                                                    unpack_opaque_flex (
                                                    input , None ));
                                           sz += fsz;
                                           v
                                       },
                                   op_errstr:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: unpack_string
                                                    ( input , None ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_friend_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_friend_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_friend_req{uuid:
                                    {
                                        let (v, fsz) =
                                            {
                                                use std::mem;
                                                let mut buf:
                                                        [u8; 16i64 as usize] =
                                                    unsafe {
                                                        mem::uninitialized()
                                                    };
                                                let sz =
                                                    try!(xdr_codec ::
                                                         unpack_opaque_array (
                                                         input , & mut buf [
                                                         .. ] , 16i64 as usize
                                                         ));
                                                (buf, sz)
                                            };
                                        sz += fsz;
                                        v
                                    },
                                hostname:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: unpack_string (
                                                 input , None ));
                                        sz += fsz;
                                        v
                                    },
                                port:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                vols:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec ::
                                                 unpack_opaque_flex (
                                                 input , None ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_friend_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_friend_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_friend_rsp{uuid:
                                    {
                                        let (v, fsz) =
                                            {
                                                use std::mem;
                                                let mut buf:
                                                        [u8; 16i64 as usize] =
                                                    unsafe {
                                                        mem::uninitialized()
                                                    };
                                                let sz =
                                                    try!(xdr_codec ::
                                                         unpack_opaque_array (
                                                         input , & mut buf [
                                                         .. ] , 16i64 as usize
                                                         ));
                                                (buf, sz)
                                            };
                                        sz += fsz;
                                        v
                                    },
                                hostname:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: unpack_string (
                                                 input , None ));
                                        sz += fsz;
                                        v
                                    },
                                op_ret:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                op_errno:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                port:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_friend_update {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_friend_update, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_friend_update{uuid:
                                       {
                                           let (v, fsz) =
                                               {
                                                   use std::mem;
                                                   let mut buf:
                                                           [u8; 16i64 as
                                                                    usize] =
                                                       unsafe {
                                                           mem::uninitialized()
                                                       };
                                                   let sz =
                                                       try!(xdr_codec ::
                                                            unpack_opaque_array
                                                            (
                                                            input , & mut buf
                                                            [ .. ] , 16i64 as
                                                            usize ));
                                                   (buf, sz)
                                               };
                                           sz += fsz;
                                           v
                                       },
                                   friends:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec ::
                                                    unpack_opaque_flex (
                                                    input , None ));
                                           sz += fsz;
                                           v
                                       },
                                   port:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for
 gd1_mgmt_friend_update_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_friend_update_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_friend_update_rsp{uuid:
                                           {
                                               let (v, fsz) =
                                                   {
                                                       use std::mem;
                                                       let mut buf:
                                                               [u8; 16i64 as
                                                                        usize] =
                                                           unsafe {
                                                               mem::uninitialized()
                                                           };
                                                       let sz =
                                                           try!(xdr_codec ::
                                                                unpack_opaque_array
                                                                (
                                                                input , & mut
                                                                buf [ .. ] ,
                                                                16i64 as usize
                                                                ));
                                                       (buf, sz)
                                                   };
                                               sz += fsz;
                                               v
                                           },
                                       op:
                                           {
                                               let (v, fsz) =
                                                   try!(xdr_codec :: Unpack ::
                                                        unpack ( input ));
                                               sz += fsz;
                                               v
                                           },
                                       op_ret:
                                           {
                                               let (v, fsz) =
                                                   try!(xdr_codec :: Unpack ::
                                                        unpack ( input ));
                                               sz += fsz;
                                               v
                                           },
                                       op_errno:
                                           {
                                               let (v, fsz) =
                                                   try!(xdr_codec :: Unpack ::
                                                        unpack ( input ));
                                               sz += fsz;
                                               v
                                           },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_probe_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_probe_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_probe_rsp{uuid:
                                   {
                                       let (v, fsz) =
                                           {
                                               use std::mem;
                                               let mut buf:
                                                       [u8; 16i64 as usize] =
                                                   unsafe {
                                                       mem::uninitialized()
                                                   };
                                               let sz =
                                                   try!(xdr_codec ::
                                                        unpack_opaque_array (
                                                        input , & mut buf [ ..
                                                        ] , 16i64 as usize ));
                                               (buf, sz)
                                           };
                                       sz += fsz;
                                       v
                                   },
                               hostname:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: unpack_string (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },
                               port:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               op_ret:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               op_errno:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               op_errstr:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: unpack_string (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_stage_op_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_stage_op_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_stage_op_req{uuid:
                                      {
                                          let (v, fsz) =
                                              {
                                                  use std::mem;
                                                  let mut buf:
                                                          [u8; 16i64 as
                                                                   usize] =
                                                      unsafe {
                                                          mem::uninitialized()
                                                      };
                                                  let sz =
                                                      try!(xdr_codec ::
                                                           unpack_opaque_array
                                                           (
                                                           input , & mut buf [
                                                           .. ] , 16i64 as
                                                           usize ));
                                                  (buf, sz)
                                              };
                                          sz += fsz;
                                          v
                                      },
                                  op:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  buf:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec ::
                                                   unpack_opaque_flex (
                                                   input , None ));
                                          sz += fsz;
                                          v
                                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_stage_op_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_stage_op_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_stage_op_rsp{uuid:
                                      {
                                          let (v, fsz) =
                                              {
                                                  use std::mem;
                                                  let mut buf:
                                                          [u8; 16i64 as
                                                                   usize] =
                                                      unsafe {
                                                          mem::uninitialized()
                                                      };
                                                  let sz =
                                                      try!(xdr_codec ::
                                                           unpack_opaque_array
                                                           (
                                                           input , & mut buf [
                                                           .. ] , 16i64 as
                                                           usize ));
                                                  (buf, sz)
                                              };
                                          sz += fsz;
                                          v
                                      },
                                  op:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  op_ret:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  op_errno:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  op_errstr:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: unpack_string
                                                   ( input , None ));
                                          sz += fsz;
                                          v
                                      },
                                  dict:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec ::
                                                   unpack_opaque_flex (
                                                   input , None ));
                                          sz += fsz;
                                          v
                                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_unfriend_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_unfriend_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_unfriend_req{uuid:
                                      {
                                          let (v, fsz) =
                                              {
                                                  use std::mem;
                                                  let mut buf:
                                                          [u8; 16i64 as
                                                                   usize] =
                                                      unsafe {
                                                          mem::uninitialized()
                                                      };
                                                  let sz =
                                                      try!(xdr_codec ::
                                                           unpack_opaque_array
                                                           (
                                                           input , & mut buf [
                                                           .. ] , 16i64 as
                                                           usize ));
                                                  (buf, sz)
                                              };
                                          sz += fsz;
                                          v
                                      },
                                  hostname:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: unpack_string
                                                   ( input , None ));
                                          sz += fsz;
                                          v
                                      },
                                  port:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_unfriend_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_unfriend_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_unfriend_rsp{uuid:
                                      {
                                          let (v, fsz) =
                                              {
                                                  use std::mem;
                                                  let mut buf:
                                                          [u8; 16i64 as
                                                                   usize] =
                                                      unsafe {
                                                          mem::uninitialized()
                                                      };
                                                  let sz =
                                                      try!(xdr_codec ::
                                                           unpack_opaque_array
                                                           (
                                                           input , & mut buf [
                                                           .. ] , 16i64 as
                                                           usize ));
                                                  (buf, sz)
                                              };
                                          sz += fsz;
                                          v
                                      },
                                  hostname:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: unpack_string
                                                   ( input , None ));
                                          sz += fsz;
                                          v
                                      },
                                  op_ret:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  op_errno:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  port:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_brick_op_req
 {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_brick_op_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_brick_op_req{uuid:
                                         {
                                             let (v, fsz) =
                                                 {
                                                     use std::mem;
                                                     let mut buf:
                                                             [u8; 16i64 as
                                                                      usize] =
                                                         unsafe {
                                                             mem::uninitialized()
                                                         };
                                                     let sz =
                                                         try!(xdr_codec ::
                                                              unpack_opaque_array
                                                              (
                                                              input , & mut
                                                              buf [ .. ] ,
                                                              16i64 as usize
                                                              ));
                                                     (buf, sz)
                                                 };
                                             sz += fsz;
                                             v
                                         },
                                     op:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     dict:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_flex (
                                                      input , None ));
                                             sz += fsz;
                                             v
                                         },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_brick_op_rsp
 {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_brick_op_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_brick_op_rsp{uuid:
                                         {
                                             let (v, fsz) =
                                                 {
                                                     use std::mem;
                                                     let mut buf:
                                                             [u8; 16i64 as
                                                                      usize] =
                                                         unsafe {
                                                             mem::uninitialized()
                                                         };
                                                     let sz =
                                                         try!(xdr_codec ::
                                                              unpack_opaque_array
                                                              (
                                                              input , & mut
                                                              buf [ .. ] ,
                                                              16i64 as usize
                                                              ));
                                                     (buf, sz)
                                                 };
                                             sz += fsz;
                                             v
                                         },
                                     op:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     op_ret:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     op_errno:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     op_errstr:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec ::
                                                      unpack_string (
                                                      input , None ));
                                             sz += fsz;
                                             v
                                         },
                                     dict:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_flex (
                                                      input , None ));
                                             sz += fsz;
                                             v
                                         },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_commit_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_commit_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_commit_req{uuid:
                                       {
                                           let (v, fsz) =
                                               {
                                                   use std::mem;
                                                   let mut buf:
                                                           [u8; 16i64 as
                                                                    usize] =
                                                       unsafe {
                                                           mem::uninitialized()
                                                       };
                                                   let sz =
                                                       try!(xdr_codec ::
                                                            unpack_opaque_array
                                                            (
                                                            input , & mut buf
                                                            [ .. ] , 16i64 as
                                                            usize ));
                                                   (buf, sz)
                                               };
                                           sz += fsz;
                                           v
                                       },
                                   op:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   dict:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec ::
                                                    unpack_opaque_flex (
                                                    input , None ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_commit_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_commit_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_commit_rsp{uuid:
                                       {
                                           let (v, fsz) =
                                               {
                                                   use std::mem;
                                                   let mut buf:
                                                           [u8; 16i64 as
                                                                    usize] =
                                                       unsafe {
                                                           mem::uninitialized()
                                                       };
                                                   let sz =
                                                       try!(xdr_codec ::
                                                            unpack_opaque_array
                                                            (
                                                            input , & mut buf
                                                            [ .. ] , 16i64 as
                                                            usize ));
                                                   (buf, sz)
                                               };
                                           sz += fsz;
                                           v
                                       },
                                   op:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   op_ret:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   op_errno:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   dict:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec ::
                                                    unpack_opaque_flex (
                                                    input , None ));
                                           sz += fsz;
                                           v
                                       },
                                   op_errstr:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: unpack_string
                                                    ( input , None ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_lock_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_lock_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_lock_req{uuid:
                                     {
                                         let (v, fsz) =
                                             {
                                                 use std::mem;
                                                 let mut buf:
                                                         [u8; 16i64 as
                                                                  usize] =
                                                     unsafe {
                                                         mem::uninitialized()
                                                     };
                                                 let sz =
                                                     try!(xdr_codec ::
                                                          unpack_opaque_array
                                                          (
                                                          input , & mut buf [
                                                          .. ] , 16i64 as
                                                          usize ));
                                                 (buf, sz)
                                             };
                                         sz += fsz;
                                         v
                                     },
                                 txn_id:
                                     {
                                         let (v, fsz) =
                                             {
                                                 use std::mem;
                                                 let mut buf:
                                                         [u8; 16i64 as
                                                                  usize] =
                                                     unsafe {
                                                         mem::uninitialized()
                                                     };
                                                 let sz =
                                                     try!(xdr_codec ::
                                                          unpack_opaque_array
                                                          (
                                                          input , & mut buf [
                                                          .. ] , 16i64 as
                                                          usize ));
                                                 (buf, sz)
                                             };
                                         sz += fsz;
                                         v
                                     },
                                 op:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec :: Unpack ::
                                                  unpack ( input ));
                                         sz += fsz;
                                         v
                                     },
                                 dict:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec ::
                                                  unpack_opaque_flex (
                                                  input , None ));
                                         sz += fsz;
                                         v
                                     },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_lock_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_lock_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_lock_rsp{uuid:
                                     {
                                         let (v, fsz) =
                                             {
                                                 use std::mem;
                                                 let mut buf:
                                                         [u8; 16i64 as
                                                                  usize] =
                                                     unsafe {
                                                         mem::uninitialized()
                                                     };
                                                 let sz =
                                                     try!(xdr_codec ::
                                                          unpack_opaque_array
                                                          (
                                                          input , & mut buf [
                                                          .. ] , 16i64 as
                                                          usize ));
                                                 (buf, sz)
                                             };
                                         sz += fsz;
                                         v
                                     },
                                 txn_id:
                                     {
                                         let (v, fsz) =
                                             {
                                                 use std::mem;
                                                 let mut buf:
                                                         [u8; 16i64 as
                                                                  usize] =
                                                     unsafe {
                                                         mem::uninitialized()
                                                     };
                                                 let sz =
                                                     try!(xdr_codec ::
                                                          unpack_opaque_array
                                                          (
                                                          input , & mut buf [
                                                          .. ] , 16i64 as
                                                          usize ));
                                                 (buf, sz)
                                             };
                                         sz += fsz;
                                         v
                                     },
                                 dict:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec ::
                                                  unpack_opaque_flex (
                                                  input , None ));
                                         sz += fsz;
                                         v
                                     },
                                 op_ret:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec :: Unpack ::
                                                  unpack ( input ));
                                         sz += fsz;
                                         v
                                     },
                                 op_errno:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec :: Unpack ::
                                                  unpack ( input ));
                                         sz += fsz;
                                         v
                                     },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_post_val_req
 {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_post_val_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_post_val_req{uuid:
                                         {
                                             let (v, fsz) =
                                                 {
                                                     use std::mem;
                                                     let mut buf:
                                                             [u8; 16i64 as
                                                                      usize] =
                                                         unsafe {
                                                             mem::uninitialized()
                                                         };
                                                     let sz =
                                                         try!(xdr_codec ::
                                                              unpack_opaque_array
                                                              (
                                                              input , & mut
                                                              buf [ .. ] ,
                                                              16i64 as usize
                                                              ));
                                                     (buf, sz)
                                                 };
                                             sz += fsz;
                                             v
                                         },
                                     op:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     op_ret:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     dict:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_flex (
                                                      input , None ));
                                             sz += fsz;
                                             v
                                         },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_post_val_rsp
 {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_post_val_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_post_val_rsp{uuid:
                                         {
                                             let (v, fsz) =
                                                 {
                                                     use std::mem;
                                                     let mut buf:
                                                             [u8; 16i64 as
                                                                      usize] =
                                                         unsafe {
                                                             mem::uninitialized()
                                                         };
                                                     let sz =
                                                         try!(xdr_codec ::
                                                              unpack_opaque_array
                                                              (
                                                              input , & mut
                                                              buf [ .. ] ,
                                                              16i64 as usize
                                                              ));
                                                     (buf, sz)
                                                 };
                                             sz += fsz;
                                             v
                                         },
                                     op:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     op_ret:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     op_errno:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     op_errstr:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec ::
                                                      unpack_string (
                                                      input , None ));
                                             sz += fsz;
                                             v
                                         },
                                     dict:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_flex (
                                                      input , None ));
                                             sz += fsz;
                                             v
                                         },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_pre_val_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_pre_val_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_pre_val_req{uuid:
                                        {
                                            let (v, fsz) =
                                                {
                                                    use std::mem;
                                                    let mut buf:
                                                            [u8; 16i64 as
                                                                     usize] =
                                                        unsafe {
                                                            mem::uninitialized()
                                                        };
                                                    let sz =
                                                        try!(xdr_codec ::
                                                             unpack_opaque_array
                                                             (
                                                             input , & mut buf
                                                             [ .. ] , 16i64 as
                                                             usize ));
                                                    (buf, sz)
                                                };
                                            sz += fsz;
                                            v
                                        },
                                    op:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec :: Unpack ::
                                                     unpack ( input ));
                                            sz += fsz;
                                            v
                                        },
                                    dict:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec ::
                                                     unpack_opaque_flex (
                                                     input , None ));
                                            sz += fsz;
                                            v
                                        },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_pre_val_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_pre_val_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_pre_val_rsp{uuid:
                                        {
                                            let (v, fsz) =
                                                {
                                                    use std::mem;
                                                    let mut buf:
                                                            [u8; 16i64 as
                                                                     usize] =
                                                        unsafe {
                                                            mem::uninitialized()
                                                        };
                                                    let sz =
                                                        try!(xdr_codec ::
                                                             unpack_opaque_array
                                                             (
                                                             input , & mut buf
                                                             [ .. ] , 16i64 as
                                                             usize ));
                                                    (buf, sz)
                                                };
                                            sz += fsz;
                                            v
                                        },
                                    op:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec :: Unpack ::
                                                     unpack ( input ));
                                            sz += fsz;
                                            v
                                        },
                                    op_ret:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec :: Unpack ::
                                                     unpack ( input ));
                                            sz += fsz;
                                            v
                                        },
                                    op_errno:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec :: Unpack ::
                                                     unpack ( input ));
                                            sz += fsz;
                                            v
                                        },
                                    op_errstr:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec ::
                                                     unpack_string (
                                                     input , None ));
                                            sz += fsz;
                                            v
                                        },
                                    dict:
                                        {
                                            let (v, fsz) =
                                                try!(xdr_codec ::
                                                     unpack_opaque_flex (
                                                     input , None ));
                                            sz += fsz;
                                            v
                                        },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_unlock_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_unlock_req, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_unlock_req{uuid:
                                       {
                                           let (v, fsz) =
                                               {
                                                   use std::mem;
                                                   let mut buf:
                                                           [u8; 16i64 as
                                                                    usize] =
                                                       unsafe {
                                                           mem::uninitialized()
                                                       };
                                                   let sz =
                                                       try!(xdr_codec ::
                                                            unpack_opaque_array
                                                            (
                                                            input , & mut buf
                                                            [ .. ] , 16i64 as
                                                            usize ));
                                                   (buf, sz)
                                               };
                                           sz += fsz;
                                           v
                                       },
                                   txn_id:
                                       {
                                           let (v, fsz) =
                                               {
                                                   use std::mem;
                                                   let mut buf:
                                                           [u8; 16i64 as
                                                                    usize] =
                                                       unsafe {
                                                           mem::uninitialized()
                                                       };
                                                   let sz =
                                                       try!(xdr_codec ::
                                                            unpack_opaque_array
                                                            (
                                                            input , & mut buf
                                                            [ .. ] , 16i64 as
                                                            usize ));
                                                   (buf, sz)
                                               };
                                           sz += fsz;
                                           v
                                       },
                                   op:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   dict:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec ::
                                                    unpack_opaque_flex (
                                                    input , None ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gd1_mgmt_v3_unlock_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gd1_mgmt_v3_unlock_rsp, usize)> {
        let mut sz = 0;
        Ok((gd1_mgmt_v3_unlock_rsp{uuid:
                                       {
                                           let (v, fsz) =
                                               {
                                                   use std::mem;
                                                   let mut buf:
                                                           [u8; 16i64 as
                                                                    usize] =
                                                       unsafe {
                                                           mem::uninitialized()
                                                       };
                                                   let sz =
                                                       try!(xdr_codec ::
                                                            unpack_opaque_array
                                                            (
                                                            input , & mut buf
                                                            [ .. ] , 16i64 as
                                                            usize ));
                                                   (buf, sz)
                                               };
                                           sz += fsz;
                                           v
                                       },
                                   txn_id:
                                       {
                                           let (v, fsz) =
                                               {
                                                   use std::mem;
                                                   let mut buf:
                                                           [u8; 16i64 as
                                                                    usize] =
                                                       unsafe {
                                                           mem::uninitialized()
                                                       };
                                                   let sz =
                                                       try!(xdr_codec ::
                                                            unpack_opaque_array
                                                            (
                                                            input , & mut buf
                                                            [ .. ] , 16i64 as
                                                            usize ));
                                                   (buf, sz)
                                               };
                                           sz += fsz;
                                           v
                                       },
                                   dict:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec ::
                                                    unpack_opaque_flex (
                                                    input , None ));
                                           sz += fsz;
                                           v
                                       },
                                   op_ret:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   op_errno:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_friends_list {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_friends_list, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf1_cli_friends_list::GF_CLI_LIST_PEERS as i32)
                    => gf1_cli_friends_list::GF_CLI_LIST_PEERS,
                    x if
                    x == (gf1_cli_friends_list::GF_CLI_LIST_POOL_NODES as i32)
                    => gf1_cli_friends_list::GF_CLI_LIST_POOL_NODES,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_fsm_log_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_fsm_log_req, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_fsm_log_req{name:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: unpack_string (
                                                 input , None ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_fsm_log_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_fsm_log_rsp, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_fsm_log_rsp{op_ret:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                op_errno:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                op_errstr:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: unpack_string (
                                                 input , None ));
                                        sz += fsz;
                                        v
                                    },
                                fsm_log:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec ::
                                                 unpack_opaque_flex (
                                                 input , None ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_get_volume {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_get_volume, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x == (gf1_cli_get_volume::GF_CLI_GET_VOLUME_ALL as i32) =>
                    gf1_cli_get_volume::GF_CLI_GET_VOLUME_ALL,
                    x if x == (gf1_cli_get_volume::GF_CLI_GET_VOLUME as i32)
                    => gf1_cli_get_volume::GF_CLI_GET_VOLUME,
                    x if
                    x == (gf1_cli_get_volume::GF_CLI_GET_NEXT_VOLUME as i32)
                    => gf1_cli_get_volume::GF_CLI_GET_NEXT_VOLUME,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_getwd_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_getwd_req, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_getwd_req{unused:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_getwd_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_getwd_rsp, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_getwd_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              wd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_string (
                                               input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_gsync_set {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_gsync_set, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x == (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_NONE as i32)
                    => gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_NONE,
                    x if
                    x ==
                        (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_START as i32)
                    => gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_START,
                    x if
                    x == (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_STOP as i32)
                    => gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_STOP,
                    x if
                    x ==
                        (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_CONFIG as
                             i32) =>
                    gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_CONFIG,
                    x if
                    x ==
                        (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_STATUS as
                             i32) =>
                    gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_STATUS,
                    x if
                    x ==
                        (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_ROTATE as
                             i32) =>
                    gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_ROTATE,
                    x if
                    x ==
                        (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_CREATE as
                             i32) =>
                    gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_CREATE,
                    x if
                    x ==
                        (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_DELETE as
                             i32) =>
                    gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_DELETE,
                    x if
                    x ==
                        (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_PAUSE as i32)
                    => gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_PAUSE,
                    x if
                    x ==
                        (gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_RESUME as
                             i32) =>
                    gf1_cli_gsync_set::GF_GSYNC_OPTION_TYPE_RESUME,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_info_op {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf1_cli_info_op, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf1_cli_info_op::GF_CLI_INFO_NONE as i32) =>
                    gf1_cli_info_op::GF_CLI_INFO_NONE,
                    x if x == (gf1_cli_info_op::GF_CLI_INFO_ALL as i32) =>
                    gf1_cli_info_op::GF_CLI_INFO_ALL,
                    x if
                    x == (gf1_cli_info_op::GF_CLI_INFO_INCREMENTAL as i32) =>
                    gf1_cli_info_op::GF_CLI_INFO_INCREMENTAL,
                    x if x == (gf1_cli_info_op::GF_CLI_INFO_CUMULATIVE as i32)
                    => gf1_cli_info_op::GF_CLI_INFO_CUMULATIVE,
                    x if x == (gf1_cli_info_op::GF_CLI_INFO_CLEAR as i32) =>
                    gf1_cli_info_op::GF_CLI_INFO_CLEAR,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_mount_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_mount_req, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_mount_req{label:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_string (
                                               input , None ));
                                      sz += fsz;
                                      v
                                  },
                              dict:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_mount_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_mount_rsp, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_mount_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              path:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_string (
                                               input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_op_flags {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_op_flags, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf1_cli_op_flags::GF_CLI_FLAG_OP_FORCE as i32)
                    => gf1_cli_op_flags::GF_CLI_FLAG_OP_FORCE,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_peer_list_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_peer_list_req, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_peer_list_req{flags:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  dict:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec ::
                                                   unpack_opaque_flex (
                                                   input , None ));
                                          sz += fsz;
                                          v
                                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_peer_list_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_peer_list_rsp, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_peer_list_rsp{op_ret:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  op_errno:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  friends:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec ::
                                                   unpack_opaque_flex (
                                                   input , None ));
                                          sz += fsz;
                                          v
                                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_snapshot {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_snapshot, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x == (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_NONE as i32)
                    => gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_NONE,
                    x if
                    x == (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_CREATE as i32)
                    => gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_CREATE,
                    x if
                    x == (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_DELETE as i32)
                    => gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_DELETE,
                    x if
                    x ==
                        (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_RESTORE as i32)
                    => gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_RESTORE,
                    x if
                    x ==
                        (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_ACTIVATE as
                             i32) =>
                    gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_ACTIVATE,
                    x if
                    x ==
                        (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_DEACTIVATE as
                             i32) =>
                    gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_DEACTIVATE,
                    x if
                    x == (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_LIST as i32)
                    => gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_LIST,
                    x if
                    x == (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_STATUS as i32)
                    => gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_STATUS,
                    x if
                    x == (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_CONFIG as i32)
                    => gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_CONFIG,
                    x if
                    x == (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_CLONE as i32)
                    => gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_CLONE,
                    x if
                    x == (gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_INFO as i32)
                    => gf1_cli_snapshot::GF_SNAP_OPTION_TYPE_INFO,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_snapshot_config {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_snapshot_config, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x ==
                        (gf1_cli_snapshot_config::GF_SNAP_CONFIG_TYPE_NONE as
                             i32) =>
                    gf1_cli_snapshot_config::GF_SNAP_CONFIG_TYPE_NONE,
                    x if
                    x ==
                        (gf1_cli_snapshot_config::GF_SNAP_CONFIG_TYPE_SET as
                             i32) =>
                    gf1_cli_snapshot_config::GF_SNAP_CONFIG_TYPE_SET,
                    x if
                    x ==
                        (gf1_cli_snapshot_config::GF_SNAP_CONFIG_DISPLAY as
                             i32) =>
                    gf1_cli_snapshot_config::GF_SNAP_CONFIG_DISPLAY,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_snapshot_delete {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_snapshot_delete, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x ==
                        (gf1_cli_snapshot_delete::GF_SNAP_DELETE_TYPE_ALL as
                             i32) =>
                    gf1_cli_snapshot_delete::GF_SNAP_DELETE_TYPE_ALL,
                    x if
                    x ==
                        (gf1_cli_snapshot_delete::GF_SNAP_DELETE_TYPE_SNAP as
                             i32) =>
                    gf1_cli_snapshot_delete::GF_SNAP_DELETE_TYPE_SNAP,
                    x if
                    x ==
                        (gf1_cli_snapshot_delete::GF_SNAP_DELETE_TYPE_VOL as
                             i32) =>
                    gf1_cli_snapshot_delete::GF_SNAP_DELETE_TYPE_VOL,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_snapshot_info {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_snapshot_info, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x == (gf1_cli_snapshot_info::GF_SNAP_INFO_TYPE_ALL as i32)
                    => gf1_cli_snapshot_info::GF_SNAP_INFO_TYPE_ALL,
                    x if
                    x ==
                        (gf1_cli_snapshot_info::GF_SNAP_INFO_TYPE_SNAP as i32)
                    => gf1_cli_snapshot_info::GF_SNAP_INFO_TYPE_SNAP,
                    x if
                    x == (gf1_cli_snapshot_info::GF_SNAP_INFO_TYPE_VOL as i32)
                    => gf1_cli_snapshot_info::GF_SNAP_INFO_TYPE_VOL,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_snapshot_status {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_snapshot_status, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x ==
                        (gf1_cli_snapshot_status::GF_SNAP_STATUS_TYPE_ALL as
                             i32) =>
                    gf1_cli_snapshot_status::GF_SNAP_STATUS_TYPE_ALL,
                    x if
                    x ==
                        (gf1_cli_snapshot_status::GF_SNAP_STATUS_TYPE_SNAP as
                             i32) =>
                    gf1_cli_snapshot_status::GF_SNAP_STATUS_TYPE_SNAP,
                    x if
                    x ==
                        (gf1_cli_snapshot_status::GF_SNAP_STATUS_TYPE_VOL as
                             i32) =>
                    gf1_cli_snapshot_status::GF_SNAP_STATUS_TYPE_VOL,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_stats_op {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_stats_op, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf1_cli_stats_op::GF_CLI_STATS_NONE as i32) =>
                    gf1_cli_stats_op::GF_CLI_STATS_NONE,
                    x if x == (gf1_cli_stats_op::GF_CLI_STATS_START as i32) =>
                    gf1_cli_stats_op::GF_CLI_STATS_START,
                    x if x == (gf1_cli_stats_op::GF_CLI_STATS_STOP as i32) =>
                    gf1_cli_stats_op::GF_CLI_STATS_STOP,
                    x if x == (gf1_cli_stats_op::GF_CLI_STATS_INFO as i32) =>
                    gf1_cli_stats_op::GF_CLI_STATS_INFO,
                    x if x == (gf1_cli_stats_op::GF_CLI_STATS_TOP as i32) =>
                    gf1_cli_stats_op::GF_CLI_STATS_TOP,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_sync_volume {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_sync_volume, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf1_cli_sync_volume::GF_CLI_SYNC_ALL as i32) =>
                    gf1_cli_sync_volume::GF_CLI_SYNC_ALL,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_top_op {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf1_cli_top_op, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf1_cli_top_op::GF_CLI_TOP_NONE as i32) =>
                    gf1_cli_top_op::GF_CLI_TOP_NONE,
                    x if x == (gf1_cli_top_op::GF_CLI_TOP_OPEN as i32) =>
                    gf1_cli_top_op::GF_CLI_TOP_OPEN,
                    x if x == (gf1_cli_top_op::GF_CLI_TOP_READ as i32) =>
                    gf1_cli_top_op::GF_CLI_TOP_READ,
                    x if x == (gf1_cli_top_op::GF_CLI_TOP_WRITE as i32) =>
                    gf1_cli_top_op::GF_CLI_TOP_WRITE,
                    x if x == (gf1_cli_top_op::GF_CLI_TOP_OPENDIR as i32) =>
                    gf1_cli_top_op::GF_CLI_TOP_OPENDIR,
                    x if x == (gf1_cli_top_op::GF_CLI_TOP_READDIR as i32) =>
                    gf1_cli_top_op::GF_CLI_TOP_READDIR,
                    x if x == (gf1_cli_top_op::GF_CLI_TOP_READ_PERF as i32) =>
                    gf1_cli_top_op::GF_CLI_TOP_READ_PERF,
                    x if x == (gf1_cli_top_op::GF_CLI_TOP_WRITE_PERF as i32)
                    => gf1_cli_top_op::GF_CLI_TOP_WRITE_PERF,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_umount_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_umount_req, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_umount_req{lazy:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               path:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: unpack_string (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cli_umount_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cli_umount_rsp, usize)> {
        let mut sz = 0;
        Ok((gf1_cli_umount_rsp{op_ret:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               op_errno:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_cluster_type {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf1_cluster_type, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf1_cluster_type::GF_CLUSTER_TYPE_NONE as i32)
                    => gf1_cluster_type::GF_CLUSTER_TYPE_NONE,
                    x if
                    x == (gf1_cluster_type::GF_CLUSTER_TYPE_STRIPE as i32) =>
                    gf1_cluster_type::GF_CLUSTER_TYPE_STRIPE,
                    x if
                    x == (gf1_cluster_type::GF_CLUSTER_TYPE_REPLICATE as i32)
                    => gf1_cluster_type::GF_CLUSTER_TYPE_REPLICATE,
                    x if
                    x ==
                        (gf1_cluster_type::GF_CLUSTER_TYPE_STRIPE_REPLICATE as
                             i32) =>
                    gf1_cluster_type::GF_CLUSTER_TYPE_STRIPE_REPLICATE,
                    x if
                    x == (gf1_cluster_type::GF_CLUSTER_TYPE_DISPERSE as i32)
                    => gf1_cluster_type::GF_CLUSTER_TYPE_DISPERSE,
                    x if x == (gf1_cluster_type::GF_CLUSTER_TYPE_TIER as i32)
                    => gf1_cluster_type::GF_CLUSTER_TYPE_TIER,
                    x if x == (gf1_cluster_type::GF_CLUSTER_TYPE_MAX as i32)
                    => gf1_cluster_type::GF_CLUSTER_TYPE_MAX,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf1_op_commands {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf1_op_commands, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf1_op_commands::GF_OP_CMD_NONE as i32) =>
                    gf1_op_commands::GF_OP_CMD_NONE,
                    x if x == (gf1_op_commands::GF_OP_CMD_START as i32) =>
                    gf1_op_commands::GF_OP_CMD_START,
                    x if x == (gf1_op_commands::GF_OP_CMD_COMMIT as i32) =>
                    gf1_op_commands::GF_OP_CMD_COMMIT,
                    x if x == (gf1_op_commands::GF_OP_CMD_STOP as i32) =>
                    gf1_op_commands::GF_OP_CMD_STOP,
                    x if x == (gf1_op_commands::GF_OP_CMD_STATUS as i32) =>
                    gf1_op_commands::GF_OP_CMD_STATUS,
                    x if x == (gf1_op_commands::GF_OP_CMD_COMMIT_FORCE as i32)
                    => gf1_op_commands::GF_OP_CMD_COMMIT_FORCE,
                    x if x == (gf1_op_commands::GF_OP_CMD_DETACH_START as i32)
                    => gf1_op_commands::GF_OP_CMD_DETACH_START,
                    x if
                    x == (gf1_op_commands::GF_OP_CMD_DETACH_COMMIT as i32) =>
                    gf1_op_commands::GF_OP_CMD_DETACH_COMMIT,
                    x if
                    x ==
                        (gf1_op_commands::GF_OP_CMD_DETACH_COMMIT_FORCE as
                             i32) =>
                    gf1_op_commands::GF_OP_CMD_DETACH_COMMIT_FORCE,
                    x if
                    x == (gf1_op_commands::GF_OP_CMD_STOP_DETACH_TIER as i32)
                    => gf1_op_commands::GF_OP_CMD_STOP_DETACH_TIER,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_bitrot_type {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_bitrot_type, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x == (gf_bitrot_type::GF_BITROT_OPTION_TYPE_NONE as i32)
                    => gf_bitrot_type::GF_BITROT_OPTION_TYPE_NONE,
                    x if
                    x == (gf_bitrot_type::GF_BITROT_OPTION_TYPE_ENABLE as i32)
                    => gf_bitrot_type::GF_BITROT_OPTION_TYPE_ENABLE,
                    x if
                    x ==
                        (gf_bitrot_type::GF_BITROT_OPTION_TYPE_DISABLE as i32)
                    => gf_bitrot_type::GF_BITROT_OPTION_TYPE_DISABLE,
                    x if
                    x ==
                        (gf_bitrot_type::GF_BITROT_OPTION_TYPE_SCRUB_THROTTLE
                             as i32) =>
                    gf_bitrot_type::GF_BITROT_OPTION_TYPE_SCRUB_THROTTLE,
                    x if
                    x ==
                        (gf_bitrot_type::GF_BITROT_OPTION_TYPE_SCRUB_FREQ as
                             i32) =>
                    gf_bitrot_type::GF_BITROT_OPTION_TYPE_SCRUB_FREQ,
                    x if
                    x == (gf_bitrot_type::GF_BITROT_OPTION_TYPE_SCRUB as i32)
                    => gf_bitrot_type::GF_BITROT_OPTION_TYPE_SCRUB,
                    x if
                    x ==
                        (gf_bitrot_type::GF_BITROT_OPTION_TYPE_EXPIRY_TIME as
                             i32) =>
                    gf_bitrot_type::GF_BITROT_OPTION_TYPE_EXPIRY_TIME,
                    x if
                    x == (gf_bitrot_type::GF_BITROT_OPTION_TYPE_MAX as i32) =>
                    gf_bitrot_type::GF_BITROT_OPTION_TYPE_MAX,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_cli_defrag_type {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_cli_defrag_type, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf_cli_defrag_type::GF_DEFRAG_CMD_START as i32)
                    => gf_cli_defrag_type::GF_DEFRAG_CMD_START,
                    x if x == (gf_cli_defrag_type::GF_DEFRAG_CMD_STOP as i32)
                    => gf_cli_defrag_type::GF_DEFRAG_CMD_STOP,
                    x if
                    x == (gf_cli_defrag_type::GF_DEFRAG_CMD_STATUS as i32) =>
                    gf_cli_defrag_type::GF_DEFRAG_CMD_STATUS,
                    x if
                    x ==
                        (gf_cli_defrag_type::GF_DEFRAG_CMD_START_LAYOUT_FIX as
                             i32) =>
                    gf_cli_defrag_type::GF_DEFRAG_CMD_START_LAYOUT_FIX,
                    x if
                    x ==
                        (gf_cli_defrag_type::GF_DEFRAG_CMD_START_FORCE as i32)
                    => gf_cli_defrag_type::GF_DEFRAG_CMD_START_FORCE,
                    x if
                    x == (gf_cli_defrag_type::GF_DEFRAG_CMD_START_TIER as i32)
                    => gf_cli_defrag_type::GF_DEFRAG_CMD_START_TIER,
                    x if
                    x ==
                        (gf_cli_defrag_type::GF_DEFRAG_CMD_STATUS_TIER as i32)
                    => gf_cli_defrag_type::GF_DEFRAG_CMD_STATUS_TIER,
                    x if
                    x ==
                        (gf_cli_defrag_type::GF_DEFRAG_CMD_START_DETACH_TIER
                             as i32) =>
                    gf_cli_defrag_type::GF_DEFRAG_CMD_START_DETACH_TIER,
                    x if
                    x ==
                        (gf_cli_defrag_type::GF_DEFRAG_CMD_STOP_DETACH_TIE as
                             i32) =>
                    gf_cli_defrag_type::GF_DEFRAG_CMD_STOP_DETACH_TIE,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_cli_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_cli_req, usize)> {
        let mut sz = 0;
        Ok((gf_cli_req{dict:
                           {
                               let (v, fsz) =
                                   try!(xdr_codec :: unpack_opaque_flex (
                                        input , None ));
                               sz += fsz;
                               v
                           },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_cli_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_cli_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_cli_rsp{op_ret:
                           {
                               let (v, fsz) =
                                   try!(xdr_codec :: Unpack :: unpack ( input
                                        ));
                               sz += fsz;
                               v
                           },
                       op_errno:
                           {
                               let (v, fsz) =
                                   try!(xdr_codec :: Unpack :: unpack ( input
                                        ));
                               sz += fsz;
                               v
                           },
                       op_errstr:
                           {
                               let (v, fsz) =
                                   try!(xdr_codec :: unpack_string (
                                        input , None ));
                               sz += fsz;
                               v
                           },
                       dict:
                           {
                               let (v, fsz) =
                                   try!(xdr_codec :: unpack_opaque_flex (
                                        input , None ));
                               sz += fsz;
                               v
                           },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_cli_status_type {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_cli_status_type, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_NONE as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_NONE,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_MEM as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_MEM,
                    x if
                    x == (gf_cli_status_type::GF_CLI_STATUS_CLIENTS as i32) =>
                    gf_cli_status_type::GF_CLI_STATUS_CLIENTS,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_INODE as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_INODE,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_FD as i32) =>
                    gf_cli_status_type::GF_CLI_STATUS_FD,
                    x if
                    x == (gf_cli_status_type::GF_CLI_STATUS_CALLPOOL as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_CALLPOOL,
                    x if
                    x == (gf_cli_status_type::GF_CLI_STATUS_DETAIL as i32) =>
                    gf_cli_status_type::GF_CLI_STATUS_DETAIL,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_TASKS as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_TASKS,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_MASK as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_MASK,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_VOL as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_VOL,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_ALL as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_ALL,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_BRICK as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_BRICK,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_NFS as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_NFS,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_SHD as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_SHD,
                    x if
                    x == (gf_cli_status_type::GF_CLI_STATUS_QUOTAD as i32) =>
                    gf_cli_status_type::GF_CLI_STATUS_QUOTAD,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_SNAPD as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_SNAPD,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_BITD as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_BITD,
                    x if x == (gf_cli_status_type::GF_CLI_STATUS_SCRUB as i32)
                    => gf_cli_status_type::GF_CLI_STATUS_SCRUB,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_common_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_common_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_common_rsp{op_ret:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          op_errno:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_defrag_status_t {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_defrag_status_t, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x ==
                        (gf_defrag_status_t::GF_DEFRAG_STATUS_NOT_STARTED as
                             i32) =>
                    gf_defrag_status_t::GF_DEFRAG_STATUS_NOT_STARTED,
                    x if
                    x == (gf_defrag_status_t::GF_DEFRAG_STATUS_STARTED as i32)
                    => gf_defrag_status_t::GF_DEFRAG_STATUS_STARTED,
                    x if
                    x == (gf_defrag_status_t::GF_DEFRAG_STATUS_STOPPED as i32)
                    => gf_defrag_status_t::GF_DEFRAG_STATUS_STOPPED,
                    x if
                    x ==
                        (gf_defrag_status_t::GF_DEFRAG_STATUS_COMPLETE as i32)
                    => gf_defrag_status_t::GF_DEFRAG_STATUS_COMPLETE,
                    x if
                    x == (gf_defrag_status_t::GF_DEFRAG_STATUS_FAILED as i32)
                    => gf_defrag_status_t::GF_DEFRAG_STATUS_FAILED,
                    x if
                    x ==
                        (gf_defrag_status_t::GF_DEFRAG_STATUS_LAYOUT_FIX_STARTED
                             as i32) =>
                    gf_defrag_status_t::GF_DEFRAG_STATUS_LAYOUT_FIX_STARTED,
                    x if
                    x ==
                        (gf_defrag_status_t::GF_DEFRAG_STATUS_LAYOUT_FIX_STOPPED
                             as i32) =>
                    gf_defrag_status_t::GF_DEFRAG_STATUS_LAYOUT_FIX_STOPPED,
                    x if
                    x ==
                        (gf_defrag_status_t::GF_DEFRAG_STATUS_LAYOUT_FIX_COMPLETE
                             as i32) =>
                    gf_defrag_status_t::GF_DEFRAG_STATUS_LAYOUT_FIX_COMPLETE,
                    x if
                    x ==
                        (gf_defrag_status_t::GF_DEFRAG_STATUS_LAYOUT_FIX_FAILED
                             as i32) =>
                    gf_defrag_status_t::GF_DEFRAG_STATUS_LAYOUT_FIX_FAILED,
                    x if
                    x == (gf_defrag_status_t::GF_DEFRAG_STATUS_MAX as i32) =>
                    gf_defrag_status_t::GF_DEFRAG_STATUS_MAX,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_dump_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_dump_req, usize)> {
        let mut sz = 0;
        Ok((gf_dump_req{gfs_id:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_dump_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_dump_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_dump_rsp{gfs_id:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        op_ret:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        op_errno:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        prog:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_event_notify_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_event_notify_req, usize)> {
        let mut sz = 0;
        Ok((gf_event_notify_req{op:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                dict:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec ::
                                                 unpack_opaque_flex (
                                                 input , None ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_event_notify_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_event_notify_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_event_notify_rsp{op_ret:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                op_errno:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                dict:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec ::
                                                 unpack_opaque_flex (
                                                 input , None ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_get_volume_info_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_get_volume_info_req, usize)> {
        let mut sz = 0;
        Ok((gf_get_volume_info_req{dict:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec ::
                                                    unpack_opaque_flex (
                                                    input , None ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_get_volume_info_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_get_volume_info_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_get_volume_info_rsp{op_ret:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   op_errno:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   op_errstr:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: unpack_string
                                                    ( input , None ));
                                           sz += fsz;
                                           v
                                       },
                                   dict:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec ::
                                                    unpack_opaque_flex (
                                                    input , None ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_getsnap_name_uuid_req
 {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_getsnap_name_uuid_req, usize)> {
        let mut sz = 0;
        Ok((gf_getsnap_name_uuid_req{dict:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_flex (
                                                      input , None ));
                                             sz += fsz;
                                             v
                                         },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_getsnap_name_uuid_rsp
 {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_getsnap_name_uuid_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_getsnap_name_uuid_rsp{op_ret:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     op_errno:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec :: Unpack ::
                                                      unpack ( input ));
                                             sz += fsz;
                                             v
                                         },
                                     op_errstr:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec ::
                                                      unpack_string (
                                                      input , None ));
                                             sz += fsz;
                                             v
                                         },
                                     dict:
                                         {
                                             let (v, fsz) =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_flex (
                                                      input , None ));
                                             sz += fsz;
                                             v
                                         },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_getspec_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_getspec_req, usize)> {
        let mut sz = 0;
        Ok((gf_getspec_req{flags:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           key:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_string (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_getspec_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_getspec_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_getspec_rsp{op_ret:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           op_errno:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           spec:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_string (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_iatt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_iatt, usize)> {
        let mut sz = 0;
        Ok((gf_iatt{ia_gfid:
                        {
                            let (v, fsz) =
                                {
                                    use std::mem;
                                    let mut buf: [u8; 16i64 as usize] =
                                        unsafe { mem::uninitialized() };
                                    let sz =
                                        try!(xdr_codec :: unpack_opaque_array
                                             (
                                             input , & mut buf [ .. ] , 16i64
                                             as usize ));
                                    (buf, sz)
                                };
                            sz += fsz;
                            v
                        },
                    ia_ino:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_dev:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    mode:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_nlink:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_uid:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_gid:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_rdev:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_size:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_blksize:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_blocks:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_atime:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_atime_nsec:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_mtime:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_mtime_nsec:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_ctime:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },
                    ia_ctime_nsec:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_log_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_log_req, usize)> {
        let mut sz = 0;
        Ok((gf_log_req{msg:
                           {
                               let (v, fsz) =
                                   try!(xdr_codec :: unpack_opaque_flex (
                                        input , None ));
                               sz += fsz;
                               v
                           },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_mgmt_hndsk_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_mgmt_hndsk_req, usize)> {
        let mut sz = 0;
        Ok((gf_mgmt_hndsk_req{hndsk:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_mgmt_hndsk_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_mgmt_hndsk_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_mgmt_hndsk_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              hndsk:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_notify_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_notify_req, usize)> {
        let mut sz = 0;
        Ok((gf_notify_req{flags:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          buf:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_string (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_notify_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_notify_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_notify_rsp{op_ret:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          op_errno:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          flags:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          buf:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_string (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_prog_detail {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_prog_detail, usize)> {
        let mut sz = 0;
        Ok((gf_prog_detail{progname:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_string (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },
                           prognum:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           progver:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           next:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_proto_flock {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_proto_flock, usize)> {
        let mut sz = 0;
        Ok((gf_proto_flock{flock_type:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           whence:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           start:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           len:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           pid:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           lk_owner:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_quota_type {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_quota_type, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x == (gf_quota_type::GF_QUOTA_OPTION_TYPE_NONE as i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_NONE,
                    x if
                    x == (gf_quota_type::GF_QUOTA_OPTION_TYPE_ENABLE as i32)
                    => gf_quota_type::GF_QUOTA_OPTION_TYPE_ENABLE,
                    x if
                    x == (gf_quota_type::GF_QUOTA_OPTION_TYPE_DISABLE as i32)
                    => gf_quota_type::GF_QUOTA_OPTION_TYPE_DISABLE,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_LIMIT_USAGE as
                             i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_LIMIT_USAGE,
                    x if
                    x == (gf_quota_type::GF_QUOTA_OPTION_TYPE_REMOVE as i32)
                    => gf_quota_type::GF_QUOTA_OPTION_TYPE_REMOVE,
                    x if
                    x == (gf_quota_type::GF_QUOTA_OPTION_TYPE_LIST as i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_LIST,
                    x if
                    x == (gf_quota_type::GF_QUOTA_OPTION_TYPE_VERSION as i32)
                    => gf_quota_type::GF_QUOTA_OPTION_TYPE_VERSION,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_ALERT_TIME as
                             i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_ALERT_TIME,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_SOFT_TIMEOUT as
                             i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_SOFT_TIMEOUT,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_HARD_TIMEOUT as
                             i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_HARD_TIMEOUT,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_DEFAULT_SOFT_LIMIT
                             as i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_DEFAULT_SOFT_LIMIT,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_VERSION_OBJECTS
                             as i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_VERSION_OBJECTS,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_LIMIT_OBJECTS as
                             i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_LIMIT_OBJECTS,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_LIST_OBJECTS as
                             i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_LIST_OBJECTS,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_REMOVE_OBJECTS as
                             i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_REMOVE_OBJECTS,
                    x if
                    x ==
                        (gf_quota_type::GF_QUOTA_OPTION_TYPE_ENABLE_OBJECTS as
                             i32) =>
                    gf_quota_type::GF_QUOTA_OPTION_TYPE_ENABLE_OBJECTS,
                    x if x == (gf_quota_type::GF_QUOTA_OPTION_TYPE_MAX as i32)
                    => gf_quota_type::GF_QUOTA_OPTION_TYPE_MAX,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_set_lk_ver_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_set_lk_ver_req, usize)> {
        let mut sz = 0;
        Ok((gf_set_lk_ver_req{uid:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_string (
                                               input , None ));
                                      sz += fsz;
                                      v
                                  },
                              lk_ver:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_set_lk_ver_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_set_lk_ver_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_set_lk_ver_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              lk_ver:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_setvolume_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_setvolume_req, usize)> {
        let mut sz = 0;
        Ok((gf_setvolume_req{dict:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_setvolume_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gf_setvolume_rsp, usize)> {
        let mut sz = 0;
        Ok((gf_setvolume_rsp{op_ret:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             op_errno:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             dict:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gf_statfs {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gf_statfs, usize)> {
        let mut sz = 0;
        Ok((gf_statfs{bsize:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      frsize:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      blocks:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      bfree:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      bavail:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      files:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      ffree:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      favail:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      fsid:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      flag:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },
                      namemax:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_access_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_access_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_access_req{gfid:
                                {
                                    let (v, fsz) =
                                        {
                                            use std::mem;
                                            let mut buf:
                                                    [u8; 16i64 as usize] =
                                                unsafe {
                                                    mem::uninitialized()
                                                };
                                            let sz =
                                                try!(xdr_codec ::
                                                     unpack_opaque_array (
                                                     input , & mut buf [ .. ]
                                                     , 16i64 as usize ));
                                            (buf, sz)
                                        };
                                    sz += fsz;
                                    v
                                },
                            mask:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for
 gfs3_cbk_cache_invalidation_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_cbk_cache_invalidation_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_cbk_cache_invalidation_req{gfid:
                                                {
                                                    let (v, fsz) =
                                                        try!(xdr_codec ::
                                                             unpack_string (
                                                             input , None ));
                                                    sz += fsz;
                                                    v
                                                },
                                            event_type:
                                                {
                                                    let (v, fsz) =
                                                        try!(xdr_codec ::
                                                             Unpack :: unpack
                                                             ( input ));
                                                    sz += fsz;
                                                    v
                                                },
                                            flags:
                                                {
                                                    let (v, fsz) =
                                                        try!(xdr_codec ::
                                                             Unpack :: unpack
                                                             ( input ));
                                                    sz += fsz;
                                                    v
                                                },
                                            expire_time_attr:
                                                {
                                                    let (v, fsz) =
                                                        try!(xdr_codec ::
                                                             Unpack :: unpack
                                                             ( input ));
                                                    sz += fsz;
                                                    v
                                                },
                                            stat:
                                                {
                                                    let (v, fsz) =
                                                        try!(xdr_codec ::
                                                             Unpack :: unpack
                                                             ( input ));
                                                    sz += fsz;
                                                    v
                                                },
                                            parent_stat:
                                                {
                                                    let (v, fsz) =
                                                        try!(xdr_codec ::
                                                             Unpack :: unpack
                                                             ( input ));
                                                    sz += fsz;
                                                    v
                                                },
                                            oldparent_stat:
                                                {
                                                    let (v, fsz) =
                                                        try!(xdr_codec ::
                                                             Unpack :: unpack
                                                             ( input ));
                                                    sz += fsz;
                                                    v
                                                },
                                            xdata:
                                                {
                                                    let (v, fsz) =
                                                        try!(xdr_codec ::
                                                             unpack_opaque_flex
                                                             ( input , None
                                                             ));
                                                    sz += fsz;
                                                    v
                                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_create_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_create_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_create_req{pargfid:
                                {
                                    let (v, fsz) =
                                        {
                                            use std::mem;
                                            let mut buf:
                                                    [u8; 16i64 as usize] =
                                                unsafe {
                                                    mem::uninitialized()
                                                };
                                            let sz =
                                                try!(xdr_codec ::
                                                     unpack_opaque_array (
                                                     input , & mut buf [ .. ]
                                                     , 16i64 as usize ));
                                            (buf, sz)
                                        };
                                    sz += fsz;
                                    v
                                },
                            flags:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            mode:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            umask:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            bname:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_string (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_create_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_create_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_create_rsp{op_ret:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            op_errno:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            stat:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            fd:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            preparent:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            postparent:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_dirlist {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_dirlist, usize)> {
        let mut sz = 0;
        Ok((gfs3_dirlist{d_ino:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: Unpack :: unpack (
                                          input ));
                                 sz += fsz;
                                 v
                             },
                         d_off:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: Unpack :: unpack (
                                          input ));
                                 sz += fsz;
                                 v
                             },
                         d_len:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: Unpack :: unpack (
                                          input ));
                                 sz += fsz;
                                 v
                             },
                         d_type:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: Unpack :: unpack (
                                          input ));
                                 sz += fsz;
                                 v
                             },
                         name:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: unpack_string (
                                          input , None ));
                                 sz += fsz;
                                 v
                             },
                         nextentry:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: Unpack :: unpack (
                                          input ));
                                 sz += fsz;
                                 v
                             },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_dirplist {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_dirplist, usize)> {
        let mut sz = 0;
        Ok((gfs3_dirplist{d_ino:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          d_off:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          d_len:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          d_type:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          name:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_string (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },
                          stat:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          dict:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },
                          nextentry:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_discard_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_discard_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_discard_req{gfid:
                                 {
                                     let (v, fsz) =
                                         {
                                             use std::mem;
                                             let mut buf:
                                                     [u8; 16i64 as usize] =
                                                 unsafe {
                                                     mem::uninitialized()
                                                 };
                                             let sz =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_array (
                                                      input , & mut buf [ .. ]
                                                      , 16i64 as usize ));
                                             (buf, sz)
                                         };
                                     sz += fsz;
                                     v
                                 },
                             fd:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             offset:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             size:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_discard_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_discard_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_discard_rsp{op_ret:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             op_errno:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             statpre:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             statpost:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_entrylk_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_entrylk_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_entrylk_req{gfid:
                                 {
                                     let (v, fsz) =
                                         {
                                             use std::mem;
                                             let mut buf:
                                                     [u8; 16i64 as usize] =
                                                 unsafe {
                                                     mem::uninitialized()
                                                 };
                                             let sz =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_array (
                                                      input , & mut buf [ .. ]
                                                      , 16i64 as usize ));
                                             (buf, sz)
                                         };
                                     sz += fsz;
                                     v
                                 },
                             cmd:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             entrylk_type:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             namelen:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             name:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_string (
                                              input , None ));
                                     sz += fsz;
                                     v
                                 },
                             volume:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_string (
                                              input , None ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fallocate_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fallocate_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_fallocate_req{gfid:
                                   {
                                       let (v, fsz) =
                                           {
                                               use std::mem;
                                               let mut buf:
                                                       [u8; 16i64 as usize] =
                                                   unsafe {
                                                       mem::uninitialized()
                                                   };
                                               let sz =
                                                   try!(xdr_codec ::
                                                        unpack_opaque_array (
                                                        input , & mut buf [ ..
                                                        ] , 16i64 as usize ));
                                               (buf, sz)
                                           };
                                       sz += fsz;
                                       v
                                   },
                               fd:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               flags:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               offset:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               size:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               xdata:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fallocate_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fallocate_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_fallocate_rsp{op_ret:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               op_errno:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               statpre:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               statpost:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               xdata:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fentrylk_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fentrylk_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_fentrylk_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              fd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              cmd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              fentrylk_type:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              namelen:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              name:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_string (
                                               input , None ));
                                      sz += fsz;
                                      v
                                  },
                              volume:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_string (
                                               input , None ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fgetxattr_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fgetxattr_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_fgetxattr_req{gfid:
                                   {
                                       let (v, fsz) =
                                           {
                                               use std::mem;
                                               let mut buf:
                                                       [u8; 16i64 as usize] =
                                                   unsafe {
                                                       mem::uninitialized()
                                                   };
                                               let sz =
                                                   try!(xdr_codec ::
                                                        unpack_opaque_array (
                                                        input , & mut buf [ ..
                                                        ] , 16i64 as usize ));
                                               (buf, sz)
                                           };
                                       sz += fsz;
                                       v
                                   },
                               fd:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               namelen:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               name:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: unpack_string (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },
                               xdata:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fgetxattr_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fgetxattr_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_fgetxattr_rsp{op_ret:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               op_errno:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               dict:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },
                               xdata:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_finodelk_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_finodelk_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_finodelk_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              fd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              cmd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              finodelk_type:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              flock:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              volume:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_string (
                                               input , None ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fremovexattr_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fremovexattr_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_fremovexattr_req{gfid:
                                      {
                                          let (v, fsz) =
                                              {
                                                  use std::mem;
                                                  let mut buf:
                                                          [u8; 16i64 as
                                                                   usize] =
                                                      unsafe {
                                                          mem::uninitialized()
                                                      };
                                                  let sz =
                                                      try!(xdr_codec ::
                                                           unpack_opaque_array
                                                           (
                                                           input , & mut buf [
                                                           .. ] , 16i64 as
                                                           usize ));
                                                  (buf, sz)
                                              };
                                          sz += fsz;
                                          v
                                      },
                                  fd:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: Unpack ::
                                                   unpack ( input ));
                                          sz += fsz;
                                          v
                                      },
                                  name:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec :: unpack_string
                                                   ( input , None ));
                                          sz += fsz;
                                          v
                                      },
                                  xdata:
                                      {
                                          let (v, fsz) =
                                              try!(xdr_codec ::
                                                   unpack_opaque_flex (
                                                   input , None ));
                                          sz += fsz;
                                          v
                                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fsetattr_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fsetattr_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_fsetattr_req{fd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              stbuf:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              valid:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fsetattr_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fsetattr_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_fsetattr_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              statpre:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              statpost:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fsetxattr_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fsetxattr_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_fsetxattr_req{gfid:
                                   {
                                       let (v, fsz) =
                                           {
                                               use std::mem;
                                               let mut buf:
                                                       [u8; 16i64 as usize] =
                                                   unsafe {
                                                       mem::uninitialized()
                                                   };
                                               let sz =
                                                   try!(xdr_codec ::
                                                        unpack_opaque_array (
                                                        input , & mut buf [ ..
                                                        ] , 16i64 as usize ));
                                               (buf, sz)
                                           };
                                       sz += fsz;
                                       v
                                   },
                               fd:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               flags:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               dict:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },
                               xdata:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fstat_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_fstat_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_fstat_req{gfid:
                               {
                                   let (v, fsz) =
                                       {
                                           use std::mem;
                                           let mut buf: [u8; 16i64 as usize] =
                                               unsafe {
                                                   mem::uninitialized()
                                               };
                                           let sz =
                                               try!(xdr_codec ::
                                                    unpack_opaque_array (
                                                    input , & mut buf [ .. ] ,
                                                    16i64 as usize ));
                                           (buf, sz)
                                       };
                                   sz += fsz;
                                   v
                               },
                           fd:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fstat_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_fstat_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_fstat_rsp{op_ret:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           op_errno:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           stat:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fsync_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_fsync_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_fsync_rsp{op_ret:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           op_errno:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           prestat:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           poststat:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fsyncdir_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fsyncdir_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_fsyncdir_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              fd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              data:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_ftruncate_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_ftruncate_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_ftruncate_req{gfid:
                                   {
                                       let (v, fsz) =
                                           {
                                               use std::mem;
                                               let mut buf:
                                                       [u8; 16i64 as usize] =
                                                   unsafe {
                                                       mem::uninitialized()
                                                   };
                                               let sz =
                                                   try!(xdr_codec ::
                                                        unpack_opaque_array (
                                                        input , & mut buf [ ..
                                                        ] , 16i64 as usize ));
                                               (buf, sz)
                                           };
                                       sz += fsz;
                                       v
                                   },
                               fd:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               offset:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               xdata:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_ftruncate_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_ftruncate_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_ftruncate_rsp{op_ret:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               op_errno:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               prestat:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               poststat:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               xdata:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fxattrop_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fxattrop_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_fxattrop_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              fd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              flags:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              dict:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_fxattrop_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_fxattrop_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_fxattrop_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              dict:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_getxattr_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_getxattr_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_getxattr_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              namelen:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              name:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_string (
                                               input , None ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_getxattr_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_getxattr_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_getxattr_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              dict:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_inodelk_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_inodelk_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_inodelk_req{gfid:
                                 {
                                     let (v, fsz) =
                                         {
                                             use std::mem;
                                             let mut buf:
                                                     [u8; 16i64 as usize] =
                                                 unsafe {
                                                     mem::uninitialized()
                                                 };
                                             let sz =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_array (
                                                      input , & mut buf [ .. ]
                                                      , 16i64 as usize ));
                                             (buf, sz)
                                         };
                                     sz += fsz;
                                     v
                                 },
                             cmd:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             inodelk_type:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             flock:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             volume:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_string (
                                              input , None ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_ipc_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_ipc_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_ipc_req{op:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: Unpack :: unpack (
                                          input ));
                                 sz += fsz;
                                 v
                             },
                         xdata:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: unpack_opaque_flex (
                                          input , None ));
                                 sz += fsz;
                                 v
                             },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_ipc_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_ipc_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_ipc_rsp{op_ret:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: Unpack :: unpack (
                                          input ));
                                 sz += fsz;
                                 v
                             },
                         op_errno:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: Unpack :: unpack (
                                          input ));
                                 sz += fsz;
                                 v
                             },
                         xdata:
                             {
                                 let (v, fsz) =
                                     try!(xdr_codec :: unpack_opaque_flex (
                                          input , None ));
                                 sz += fsz;
                                 v
                             },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_link_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_link_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_link_req{oldgfid:
                              {
                                  let (v, fsz) =
                                      {
                                          use std::mem;
                                          let mut buf: [u8; 16i64 as usize] =
                                              unsafe { mem::uninitialized() };
                                          let sz =
                                              try!(xdr_codec ::
                                                   unpack_opaque_array (
                                                   input , & mut buf [ .. ] ,
                                                   16i64 as usize ));
                                          (buf, sz)
                                      };
                                  sz += fsz;
                                  v
                              },
                          newgfid:
                              {
                                  let (v, fsz) =
                                      {
                                          use std::mem;
                                          let mut buf: [u8; 16i64 as usize] =
                                              unsafe { mem::uninitialized() };
                                          let sz =
                                              try!(xdr_codec ::
                                                   unpack_opaque_array (
                                                   input , & mut buf [ .. ] ,
                                                   16i64 as usize ));
                                          (buf, sz)
                                      };
                                  sz += fsz;
                                  v
                              },
                          newbname:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_string (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_link_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_link_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_link_rsp{op_ret:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          op_errno:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          stat:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          preparent:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          postparent:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_lk_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_lk_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_lk_req{gfid:
                            {
                                let (v, fsz) =
                                    {
                                        use std::mem;
                                        let mut buf: [u8; 16i64 as usize] =
                                            unsafe { mem::uninitialized() };
                                        let sz =
                                            try!(xdr_codec ::
                                                 unpack_opaque_array (
                                                 input , & mut buf [ .. ] ,
                                                 16i64 as usize ));
                                        (buf, sz)
                                    };
                                sz += fsz;
                                v
                            },
                        fd:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        cmd:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        lk_type:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        flock:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        xdata:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: unpack_opaque_flex (
                                         input , None ));
                                sz += fsz;
                                v
                            },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_lk_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_lk_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_lk_rsp{op_ret:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        op_errno:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        flock:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        xdata:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: unpack_opaque_flex (
                                         input , None ));
                                sz += fsz;
                                v
                            },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_lookup_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_lookup_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_lookup_req{gfid:
                                {
                                    let (v, fsz) =
                                        {
                                            use std::mem;
                                            let mut buf:
                                                    [u8; 16i64 as usize] =
                                                unsafe {
                                                    mem::uninitialized()
                                                };
                                            let sz =
                                                try!(xdr_codec ::
                                                     unpack_opaque_array (
                                                     input , & mut buf [ .. ]
                                                     , 16i64 as usize ));
                                            (buf, sz)
                                        };
                                    sz += fsz;
                                    v
                                },
                            pargfid:
                                {
                                    let (v, fsz) =
                                        {
                                            use std::mem;
                                            let mut buf:
                                                    [u8; 16i64 as usize] =
                                                unsafe {
                                                    mem::uninitialized()
                                                };
                                            let sz =
                                                try!(xdr_codec ::
                                                     unpack_opaque_array (
                                                     input , & mut buf [ .. ]
                                                     , 16i64 as usize ));
                                            (buf, sz)
                                        };
                                    sz += fsz;
                                    v
                                },
                            flags:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            bname:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_string (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_lookup_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_lookup_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_lookup_rsp{op_ret:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            op_errno:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            stat:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            postparent:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_mkdir_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_mkdir_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_mkdir_req{pargfid:
                               {
                                   let (v, fsz) =
                                       {
                                           use std::mem;
                                           let mut buf: [u8; 16i64 as usize] =
                                               unsafe {
                                                   mem::uninitialized()
                                               };
                                           let sz =
                                               try!(xdr_codec ::
                                                    unpack_opaque_array (
                                                    input , & mut buf [ .. ] ,
                                                    16i64 as usize ));
                                           (buf, sz)
                                       };
                                   sz += fsz;
                                   v
                               },
                           mode:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           umask:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           bname:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_string (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_mkdir_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_mkdir_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_mkdir_rsp{op_ret:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           op_errno:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           stat:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           preparent:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           postparent:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_mknod_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_mknod_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_mknod_req{pargfid:
                               {
                                   let (v, fsz) =
                                       {
                                           use std::mem;
                                           let mut buf: [u8; 16i64 as usize] =
                                               unsafe {
                                                   mem::uninitialized()
                                               };
                                           let sz =
                                               try!(xdr_codec ::
                                                    unpack_opaque_array (
                                                    input , & mut buf [ .. ] ,
                                                    16i64 as usize ));
                                           (buf, sz)
                                       };
                                   sz += fsz;
                                   v
                               },
                           dev:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           mode:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           umask:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           bname:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_string (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_mknod_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_mknod_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_mknod_rsp{op_ret:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           op_errno:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           stat:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           preparent:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           postparent:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_open_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_open_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_open_req{gfid:
                              {
                                  let (v, fsz) =
                                      {
                                          use std::mem;
                                          let mut buf: [u8; 16i64 as usize] =
                                              unsafe { mem::uninitialized() };
                                          let sz =
                                              try!(xdr_codec ::
                                                   unpack_opaque_array (
                                                   input , & mut buf [ .. ] ,
                                                   16i64 as usize ));
                                          (buf, sz)
                                      };
                                  sz += fsz;
                                  v
                              },
                          flags:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_open_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_open_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_open_rsp{op_ret:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          op_errno:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          fd:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_opendir_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_opendir_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_opendir_req{gfid:
                                 {
                                     let (v, fsz) =
                                         {
                                             use std::mem;
                                             let mut buf:
                                                     [u8; 16i64 as usize] =
                                                 unsafe {
                                                     mem::uninitialized()
                                                 };
                                             let sz =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_array (
                                                      input , & mut buf [ .. ]
                                                      , 16i64 as usize ));
                                             (buf, sz)
                                         };
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_opendir_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_opendir_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_opendir_rsp{op_ret:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             op_errno:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             fd:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_rchecksum_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_rchecksum_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_rchecksum_req{fd:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               offset:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               len:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               xdata:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_rchecksum_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_rchecksum_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_rchecksum_rsp{op_ret:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               op_errno:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               weak_checksum:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec :: Unpack :: unpack
                                                ( input ));
                                       sz += fsz;
                                       v
                                   },
                               strong_checksum:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },
                               xdata:
                                   {
                                       let (v, fsz) =
                                           try!(xdr_codec ::
                                                unpack_opaque_flex (
                                                input , None ));
                                       sz += fsz;
                                       v
                                   },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_read_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_read_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_read_req{gfid:
                              {
                                  let (v, fsz) =
                                      {
                                          use std::mem;
                                          let mut buf: [u8; 16i64 as usize] =
                                              unsafe { mem::uninitialized() };
                                          let sz =
                                              try!(xdr_codec ::
                                                   unpack_opaque_array (
                                                   input , & mut buf [ .. ] ,
                                                   16i64 as usize ));
                                          (buf, sz)
                                      };
                                  sz += fsz;
                                  v
                              },
                          fd:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          offset:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          size:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          flag:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_read_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_read_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_read_rsp{op_ret:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          op_errno:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          stat:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          size:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_readdir_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_readdir_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_readdir_req{gfid:
                                 {
                                     let (v, fsz) =
                                         {
                                             use std::mem;
                                             let mut buf:
                                                     [u8; 16i64 as usize] =
                                                 unsafe {
                                                     mem::uninitialized()
                                                 };
                                             let sz =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_array (
                                                      input , & mut buf [ .. ]
                                                      , 16i64 as usize ));
                                             (buf, sz)
                                         };
                                     sz += fsz;
                                     v
                                 },
                             fd:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             offset:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             size:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_readdir_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_readdir_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_readdir_rsp{op_ret:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             op_errno:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             reply:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_readdirp_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_readdirp_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_readdirp_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              fd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              offset:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              size:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              dict:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_readdirp_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_readdirp_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_readdirp_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              reply:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_readlink_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_readlink_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_readlink_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              size:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_readlink_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_readlink_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_readlink_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              buf:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              path:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_string (
                                               input , None ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_release_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_release_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_release_req{gfid:
                                 {
                                     let (v, fsz) =
                                         {
                                             use std::mem;
                                             let mut buf:
                                                     [u8; 16i64 as usize] =
                                                 unsafe {
                                                     mem::uninitialized()
                                                 };
                                             let sz =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_array (
                                                      input , & mut buf [ .. ]
                                                      , 16i64 as usize ));
                                             (buf, sz)
                                         };
                                     sz += fsz;
                                     v
                                 },
                             fd:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_releasedir_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_releasedir_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_releasedir_req{gfid:
                                    {
                                        let (v, fsz) =
                                            {
                                                use std::mem;
                                                let mut buf:
                                                        [u8; 16i64 as usize] =
                                                    unsafe {
                                                        mem::uninitialized()
                                                    };
                                                let sz =
                                                    try!(xdr_codec ::
                                                         unpack_opaque_array (
                                                         input , & mut buf [
                                                         .. ] , 16i64 as usize
                                                         ));
                                                (buf, sz)
                                            };
                                        sz += fsz;
                                        v
                                    },
                                fd:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                xdata:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec ::
                                                 unpack_opaque_flex (
                                                 input , None ));
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_removexattr_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_removexattr_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_removexattr_req{gfid:
                                     {
                                         let (v, fsz) =
                                             {
                                                 use std::mem;
                                                 let mut buf:
                                                         [u8; 16i64 as
                                                                  usize] =
                                                     unsafe {
                                                         mem::uninitialized()
                                                     };
                                                 let sz =
                                                     try!(xdr_codec ::
                                                          unpack_opaque_array
                                                          (
                                                          input , & mut buf [
                                                          .. ] , 16i64 as
                                                          usize ));
                                                 (buf, sz)
                                             };
                                         sz += fsz;
                                         v
                                     },
                                 name:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec :: unpack_string (
                                                  input , None ));
                                         sz += fsz;
                                         v
                                     },
                                 xdata:
                                     {
                                         let (v, fsz) =
                                             try!(xdr_codec ::
                                                  unpack_opaque_flex (
                                                  input , None ));
                                         sz += fsz;
                                         v
                                     },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_rename_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_rename_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_rename_req{oldgfid:
                                {
                                    let (v, fsz) =
                                        {
                                            use std::mem;
                                            let mut buf:
                                                    [u8; 16i64 as usize] =
                                                unsafe {
                                                    mem::uninitialized()
                                                };
                                            let sz =
                                                try!(xdr_codec ::
                                                     unpack_opaque_array (
                                                     input , & mut buf [ .. ]
                                                     , 16i64 as usize ));
                                            (buf, sz)
                                        };
                                    sz += fsz;
                                    v
                                },
                            newgfid:
                                {
                                    let (v, fsz) =
                                        {
                                            use std::mem;
                                            let mut buf:
                                                    [u8; 16i64 as usize] =
                                                unsafe {
                                                    mem::uninitialized()
                                                };
                                            let sz =
                                                try!(xdr_codec ::
                                                     unpack_opaque_array (
                                                     input , & mut buf [ .. ]
                                                     , 16i64 as usize ));
                                            (buf, sz)
                                        };
                                    sz += fsz;
                                    v
                                },
                            oldbname:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_string (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },
                            newbname:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_string (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_rename_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_rename_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_rename_rsp{op_ret:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            op_errno:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            stat:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            preoldparent:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            postoldparent:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            prenewparent:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            postnewparent:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_rmdir_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_rmdir_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_rmdir_req{pargfid:
                               {
                                   let (v, fsz) =
                                       {
                                           use std::mem;
                                           let mut buf: [u8; 16i64 as usize] =
                                               unsafe {
                                                   mem::uninitialized()
                                               };
                                           let sz =
                                               try!(xdr_codec ::
                                                    unpack_opaque_array (
                                                    input , & mut buf [ .. ] ,
                                                    16i64 as usize ));
                                           (buf, sz)
                                       };
                                   sz += fsz;
                                   v
                               },
                           xflags:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           bname:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_string (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_rmdir_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_rmdir_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_rmdir_rsp{op_ret:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           op_errno:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           preparent:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           postparent:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_setattr_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_setattr_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_setattr_req{gfid:
                                 {
                                     let (v, fsz) =
                                         {
                                             use std::mem;
                                             let mut buf:
                                                     [u8; 16i64 as usize] =
                                                 unsafe {
                                                     mem::uninitialized()
                                                 };
                                             let sz =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_array (
                                                      input , & mut buf [ .. ]
                                                      , 16i64 as usize ));
                                             (buf, sz)
                                         };
                                     sz += fsz;
                                     v
                                 },
                             stbuf:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             valid:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_setattr_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_setattr_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_setattr_rsp{op_ret:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             op_errno:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             statpre:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             statpost:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_setxattr_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_setxattr_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_setxattr_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              flags:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              dict:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_stat_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_stat_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_stat_req{gfid:
                              {
                                  let (v, fsz) =
                                      {
                                          use std::mem;
                                          let mut buf: [u8; 16i64 as usize] =
                                              unsafe { mem::uninitialized() };
                                          let sz =
                                              try!(xdr_codec ::
                                                   unpack_opaque_array (
                                                   input , & mut buf [ .. ] ,
                                                   16i64 as usize ));
                                          (buf, sz)
                                      };
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_stat_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_stat_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_stat_rsp{op_ret:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          op_errno:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          stat:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          xdata:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_opaque_flex (
                                           input , None ));
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_statfs_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_statfs_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_statfs_req{gfid:
                                {
                                    let (v, fsz) =
                                        {
                                            use std::mem;
                                            let mut buf:
                                                    [u8; 16i64 as usize] =
                                                unsafe {
                                                    mem::uninitialized()
                                                };
                                            let sz =
                                                try!(xdr_codec ::
                                                     unpack_opaque_array (
                                                     input , & mut buf [ .. ]
                                                     , 16i64 as usize ));
                                            (buf, sz)
                                        };
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_statfs_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_statfs_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_statfs_rsp{op_ret:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            op_errno:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            statfs:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_symlink_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_symlink_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_symlink_req{pargfid:
                                 {
                                     let (v, fsz) =
                                         {
                                             use std::mem;
                                             let mut buf:
                                                     [u8; 16i64 as usize] =
                                                 unsafe {
                                                     mem::uninitialized()
                                                 };
                                             let sz =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_array (
                                                      input , & mut buf [ .. ]
                                                      , 16i64 as usize ));
                                             (buf, sz)
                                         };
                                     sz += fsz;
                                     v
                                 },
                             bname:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_string (
                                              input , None ));
                                     sz += fsz;
                                     v
                                 },
                             umask:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             linkname:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_string (
                                              input , None ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_symlink_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_symlink_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_symlink_rsp{op_ret:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             op_errno:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             stat:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             preparent:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             postparent:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_truncate_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_truncate_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_truncate_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              offset:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_truncate_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_truncate_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_truncate_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              prestat:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              poststat:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_unlink_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_unlink_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_unlink_req{pargfid:
                                {
                                    let (v, fsz) =
                                        {
                                            use std::mem;
                                            let mut buf:
                                                    [u8; 16i64 as usize] =
                                                unsafe {
                                                    mem::uninitialized()
                                                };
                                            let sz =
                                                try!(xdr_codec ::
                                                     unpack_opaque_array (
                                                     input , & mut buf [ .. ]
                                                     , 16i64 as usize ));
                                            (buf, sz)
                                        };
                                    sz += fsz;
                                    v
                                },
                            bname:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_string (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },
                            xflags:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_unlink_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_unlink_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_unlink_rsp{op_ret:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            op_errno:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            preparent:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            postparent:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            xdata:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_opaque_flex (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_write_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_write_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_write_req{gfid:
                               {
                                   let (v, fsz) =
                                       {
                                           use std::mem;
                                           let mut buf: [u8; 16i64 as usize] =
                                               unsafe {
                                                   mem::uninitialized()
                                               };
                                           let sz =
                                               try!(xdr_codec ::
                                                    unpack_opaque_array (
                                                    input , & mut buf [ .. ] ,
                                                    16i64 as usize ));
                                           (buf, sz)
                                       };
                                   sz += fsz;
                                   v
                               },
                           fd:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           offset:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           size:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           flag:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_write_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(gfs3_write_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_write_rsp{op_ret:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           op_errno:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           prestat:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           poststat:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: Unpack :: unpack (
                                            input ));
                                   sz += fsz;
                                   v
                               },
                           xdata:
                               {
                                   let (v, fsz) =
                                       try!(xdr_codec :: unpack_opaque_flex (
                                            input , None ));
                                   sz += fsz;
                                   v
                               },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_xattrop_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_xattrop_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_xattrop_req{gfid:
                                 {
                                     let (v, fsz) =
                                         {
                                             use std::mem;
                                             let mut buf:
                                                     [u8; 16i64 as usize] =
                                                 unsafe {
                                                     mem::uninitialized()
                                                 };
                                             let sz =
                                                 try!(xdr_codec ::
                                                      unpack_opaque_array (
                                                      input , & mut buf [ .. ]
                                                      , 16i64 as usize ));
                                             (buf, sz)
                                         };
                                     sz += fsz;
                                     v
                                 },
                             flags:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             dict:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_xattrop_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_xattrop_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_xattrop_rsp{op_ret:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             op_errno:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             dict:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },
                             xdata:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_opaque_flex
                                              ( input , None ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_zerofill_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_zerofill_req, usize)> {
        let mut sz = 0;
        Ok((gfs3_zerofill_req{gfid:
                                  {
                                      let (v, fsz) =
                                          {
                                              use std::mem;
                                              let mut buf:
                                                      [u8; 16i64 as usize] =
                                                  unsafe {
                                                      mem::uninitialized()
                                                  };
                                              let sz =
                                                  try!(xdr_codec ::
                                                       unpack_opaque_array (
                                                       input , & mut buf [ ..
                                                       ] , 16i64 as usize ));
                                              (buf, sz)
                                          };
                                      sz += fsz;
                                      v
                                  },
                              fd:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              offset:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              size:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for gfs3_zerofill_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(gfs3_zerofill_rsp, usize)> {
        let mut sz = 0;
        Ok((gfs3_zerofill_rsp{op_ret:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              op_errno:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              statpre:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              statpost:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: Unpack :: unpack (
                                               input ));
                                      sz += fsz;
                                      v
                                  },
                              xdata:
                                  {
                                      let (v, fsz) =
                                          try!(xdr_codec :: unpack_opaque_flex
                                               ( input , None ));
                                      sz += fsz;
                                      v
                                  },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for glusterd_volume_status {
    #[inline]
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(glusterd_volume_status, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if
                    x == (glusterd_volume_status::GLUSTERD_STATUS_NONE as i32)
                    => glusterd_volume_status::GLUSTERD_STATUS_NONE,
                    x if
                    x ==
                        (glusterd_volume_status::GLUSTERD_STATUS_STARTED as
                             i32) =>
                    glusterd_volume_status::GLUSTERD_STATUS_STARTED,
                    x if
                    x ==
                        (glusterd_volume_status::GLUSTERD_STATUS_STOPPED as
                             i32) =>
                    glusterd_volume_status::GLUSTERD_STATUS_STOPPED,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for mon {
    fn unpack(input: &mut In) -> xdr_codec::Result<(mon, usize)> {
        let mut sz = 0;
        Ok((mon{mon_id:
                    {
                        let (v, fsz) =
                            try!(xdr_codec :: Unpack :: unpack ( input ));
                        sz += fsz;
                        v
                    },
                private:
                    {
                        let (v, fsz) =
                            {
                                use std::mem;
                                let mut buf: [u8; 16i64 as usize] =
                                    unsafe { mem::uninitialized() };
                                let sz =
                                    try!(xdr_codec :: unpack_opaque_array (
                                         input , & mut buf [ .. ] , 16i64 as
                                         usize ));
                                (buf, sz)
                            };
                        sz += fsz;
                        v
                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for mon_id {
    fn unpack(input: &mut In) -> xdr_codec::Result<(mon_id, usize)> {
        let mut sz = 0;
        Ok((mon_id{mon_name:
                       {
                           let (v, fsz) =
                               try!(xdr_codec :: unpack_string (
                                    input , Some ( SM_MAXSTRLEN as usize ) ));
                           sz += fsz;
                           v
                       },
                   my_id:
                       {
                           let (v, fsz) =
                               try!(xdr_codec :: Unpack :: unpack ( input ));
                           sz += fsz;
                           v
                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for my_id {
    fn unpack(input: &mut In) -> xdr_codec::Result<(my_id, usize)> {
        let mut sz = 0;
        Ok((my_id{my_name:
                      {
                          let (v, fsz) =
                              try!(xdr_codec :: unpack_string (
                                   input , Some ( SM_MAXSTRLEN as usize ) ));
                          sz += fsz;
                          v
                      },
                  my_prog:
                      {
                          let (v, fsz) =
                              try!(xdr_codec :: Unpack :: unpack ( input ));
                          sz += fsz;
                          v
                      },
                  my_vers:
                      {
                          let (v, fsz) =
                              try!(xdr_codec :: Unpack :: unpack ( input ));
                          sz += fsz;
                          v
                      },
                  my_proc:
                      {
                          let (v, fsz) =
                              try!(xdr_codec :: Unpack :: unpack ( input ));
                          sz += fsz;
                          v
                      },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for nlm4_freeallargs {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(nlm4_freeallargs, usize)> {
        let mut sz = 0;
        Ok((nlm4_freeallargs{name:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_string (
                                              input , Some (
                                              LM_MAXSTRLEN as usize ) ));
                                     sz += fsz;
                                     v
                                 },
                             state:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for nlm4_stat {
    fn unpack(input: &mut In) -> xdr_codec::Result<(nlm4_stat, usize)> {
        let mut sz = 0;
        Ok((nlm4_stat{stat:
                          {
                              let (v, fsz) =
                                  try!(xdr_codec :: Unpack :: unpack ( input
                                       ));
                              sz += fsz;
                              v
                          },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for nlm4_stats {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(nlm4_stats, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (nlm4_stats::nlm4_granted as i32) =>
                    nlm4_stats::nlm4_granted,
                    x if x == (nlm4_stats::nlm4_denied as i32) =>
                    nlm4_stats::nlm4_denied,
                    x if x == (nlm4_stats::nlm4_denied_nolock as i32) =>
                    nlm4_stats::nlm4_denied_nolock,
                    x if x == (nlm4_stats::nlm4_blocked as i32) =>
                    nlm4_stats::nlm4_blocked,
                    x if x == (nlm4_stats::nlm4_denied_grace_period as i32) =>
                    nlm4_stats::nlm4_denied_grace_period,
                    x if x == (nlm4_stats::nlm4_deadlck as i32) =>
                    nlm4_stats::nlm4_deadlck,
                    x if x == (nlm4_stats::nlm4_rofs as i32) =>
                    nlm4_stats::nlm4_rofs,
                    x if x == (nlm4_stats::nlm4_stale_fh as i32) =>
                    nlm4_stats::nlm4_stale_fh,
                    x if x == (nlm4_stats::nlm4_fbig as i32) =>
                    nlm4_stats::nlm4_fbig,
                    x if x == (nlm4_stats::nlm4_failed as i32) =>
                    nlm4_stats::nlm4_failed,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for nlm_sm_status {
    fn unpack(input: &mut In) -> xdr_codec::Result<(nlm_sm_status, usize)> {
        let mut sz = 0;
        Ok((nlm_sm_status{mon_name:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: unpack_string (
                                           input , Some (
                                           LM_MAXSTRLEN as usize ) ));
                                  sz += fsz;
                                  v
                              },
                          state:
                              {
                                  let (v, fsz) =
                                      try!(xdr_codec :: Unpack :: unpack (
                                           input ));
                                  sz += fsz;
                                  v
                              },
                          private:
                              {
                                  let (v, fsz) =
                                      {
                                          use std::mem;
                                          let mut buf: [u8; 16i64 as usize] =
                                              unsafe { mem::uninitialized() };
                                          let sz =
                                              try!(xdr_codec ::
                                                   unpack_opaque_array (
                                                   input , & mut buf [ .. ] ,
                                                   16i64 as usize ));
                                          (buf, sz)
                                      };
                                  sz += fsz;
                                  v
                              },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for nsm_callback_status {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(nsm_callback_status, usize)> {
        let mut sz = 0;
        Ok((nsm_callback_status{mon_name:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: unpack_string (
                                                 input , Some (
                                                 SM_MAXSTRLEN as usize ) ));
                                        sz += fsz;
                                        v
                                    },
                                state:
                                    {
                                        let (v, fsz) =
                                            try!(xdr_codec :: Unpack :: unpack
                                                 ( input ));
                                        sz += fsz;
                                        v
                                    },
                                private:
                                    {
                                        let (v, fsz) =
                                            {
                                                use std::mem;
                                                let mut buf:
                                                        [u8; 16i64 as usize] =
                                                    unsafe {
                                                        mem::uninitialized()
                                                    };
                                                let sz =
                                                    try!(xdr_codec ::
                                                         unpack_opaque_array (
                                                         input , & mut buf [
                                                         .. ] , 16i64 as usize
                                                         ));
                                                (buf, sz)
                                            };
                                        sz += fsz;
                                        v
                                    },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_brick_by_port_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(pmap_brick_by_port_req, usize)> {
        let mut sz = 0;
        Ok((pmap_brick_by_port_req{port:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_brick_by_port_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(pmap_brick_by_port_rsp, usize)> {
        let mut sz = 0;
        Ok((pmap_brick_by_port_rsp{op_ret:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   op_errno:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   status:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   brick:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: unpack_string
                                                    ( input , None ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_port_by_brick_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(pmap_port_by_brick_req, usize)> {
        let mut sz = 0;
        Ok((pmap_port_by_brick_req{brick:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: unpack_string
                                                    ( input , None ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_port_by_brick_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(pmap_port_by_brick_rsp, usize)> {
        let mut sz = 0;
        Ok((pmap_port_by_brick_rsp{op_ret:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   op_errno:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   status:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },
                                   port:
                                       {
                                           let (v, fsz) =
                                               try!(xdr_codec :: Unpack ::
                                                    unpack ( input ));
                                           sz += fsz;
                                           v
                                       },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_signin_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(pmap_signin_req, usize)> {
        let mut sz = 0;
        Ok((pmap_signin_req{brick:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_string (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },
                            port:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_signin_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(pmap_signin_rsp, usize)> {
        let mut sz = 0;
        Ok((pmap_signin_rsp{op_ret:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            op_errno:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_signout_req {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(pmap_signout_req, usize)> {
        let mut sz = 0;
        Ok((pmap_signout_req{brick:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: unpack_string (
                                              input , None ));
                                     sz += fsz;
                                     v
                                 },
                             port:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             rdma_port:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_signout_rsp {
    fn unpack(input: &mut In)
     -> xdr_codec::Result<(pmap_signout_rsp, usize)> {
        let mut sz = 0;
        Ok((pmap_signout_rsp{op_ret:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },
                             op_errno:
                                 {
                                     let (v, fsz) =
                                         try!(xdr_codec :: Unpack :: unpack (
                                              input ));
                                     sz += fsz;
                                     v
                                 },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_signup_req {
    fn unpack(input: &mut In) -> xdr_codec::Result<(pmap_signup_req, usize)> {
        let mut sz = 0;
        Ok((pmap_signup_req{brick:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: unpack_string (
                                             input , None ));
                                    sz += fsz;
                                    v
                                },
                            port:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for pmap_signup_rsp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(pmap_signup_rsp, usize)> {
        let mut sz = 0;
        Ok((pmap_signup_rsp{op_ret:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },
                            op_errno:
                                {
                                    let (v, fsz) =
                                        try!(xdr_codec :: Unpack :: unpack (
                                             input ));
                                    sz += fsz;
                                    v
                                },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for res {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(res, usize)> {
        let mut sz = 0;
        Ok(({
                let (e, esz): (i32, _) =
                    try!(xdr_codec :: Unpack :: unpack ( input ));
                sz += esz;
                match e {
                    x if x == (res::STAT_SUCC as i32) => res::STAT_SUCC,
                    x if x == (res::STAT_FAIL as i32) => res::STAT_FAIL,
                    _ => return Err(xdr_codec::Error::invalidenum()),
                }
            }, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for sm_name {
    fn unpack(input: &mut In) -> xdr_codec::Result<(sm_name, usize)> {
        let mut sz = 0;
        Ok((sm_name{mon_name:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: unpack_string (
                                     input , Some ( SM_MAXSTRLEN as usize )
                                     ));
                            sz += fsz;
                            v
                        },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for sm_stat {
    fn unpack(input: &mut In) -> xdr_codec::Result<(sm_stat, usize)> {
        let mut sz = 0;
        Ok((sm_stat{state:
                        {
                            let (v, fsz) =
                                try!(xdr_codec :: Unpack :: unpack ( input ));
                            sz += fsz;
                            v
                        },}, sz))
    }
}

impl <In: xdr_codec::Read> xdr_codec::Unpack<In> for sm_stat_res {
    fn unpack(input: &mut In) -> xdr_codec::Result<(sm_stat_res, usize)> {
        let mut sz = 0;
        Ok((sm_stat_res{res_stat:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },
                        state:
                            {
                                let (v, fsz) =
                                    try!(xdr_codec :: Unpack :: unpack ( input
                                         ));
                                sz += fsz;
                                v
                            },}, sz))
    }
}
