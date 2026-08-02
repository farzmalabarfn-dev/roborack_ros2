#!/usr/bin/env python3

import rclpy
from rclpy.node import Node

from geometry_msgs.msg import Twist, TwistStamped


class CmdVelBridge(Node):

    def __init__(self):
        super().__init__("cmd_vel_bridge")

        self.publisher = self.create_publisher(
            TwistStamped,
            "/diff_drive_controller/cmd_vel",
            10,
        )

        self.subscription = self.create_subscription(
            Twist,
            "/cmd_vel",
            self.cmd_vel_callback,
            10,
        )

        self.get_logger().info("cmd_vel_bridge started")

    def cmd_vel_callback(self, msg: Twist):
        stamped = TwistStamped()

        stamped.header.stamp = self.get_clock().now().to_msg()
        stamped.header.frame_id = "base_link"

        stamped.twist = msg

        self.publisher.publish(stamped)

        self.get_logger().info(
            f"Forwarded cmd_vel: linear={msg.linear.x:.2f}, angular={msg.angular.z:.2f}"
        )


def main(args=None):
    rclpy.init(args=args)

    node = CmdVelBridge()

    rclpy.spin(node)

    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()