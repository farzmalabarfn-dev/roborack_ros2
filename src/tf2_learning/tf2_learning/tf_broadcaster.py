import rclpy

from rclpy.node import Node

from tf2_ros import TransformBroadcaster

from geometry_msgs.msg import TransformStamped


class TFBroadcaster(Node):

    def __init__(self):

        super().__init__('tf_broadcaster')

        self.broadcaster = TransformBroadcaster(self)

        self.x = 0.0

        self.timer = self.create_timer(0.1, self.broadcast_tf)

    def broadcast_tf(self):

        t = TransformStamped()

        t.header.stamp = self.get_clock().now().to_msg()

        t.header.frame_id = 'world'

        t.child_frame_id = 'base_link'

        self.x += 0.1

        t.transform.translation.x = self.x
        t.transform.translation.y = 1.0
        t.transform.translation.z = 0.0

        t.transform.rotation.x = 0.0
        t.transform.rotation.y = 0.0
        t.transform.rotation.z = 0.0
        t.transform.rotation.w = 1.0

        self.broadcaster.sendTransform(t)


def main(args=None):

    rclpy.init(args=args)

    node = TFBroadcaster()

    rclpy.spin(node)

    rclpy.shutdown()


if __name__ == '__main__':
    main()