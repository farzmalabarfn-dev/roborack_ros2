import math
import random

import rclpy
from rclpy.node import Node

from geometry_msgs.msg import Point
from std_msgs.msg import Float32


class RSSIGenerator(Node):

    def __init__(self):
        super().__init__('rssi_generator')

        # -----------------------------
        # Robot Position
        # -----------------------------
        self.robot_x = 0.0
        self.robot_y = 0.0

        # -----------------------------
        # Beacon Positions
        # -----------------------------
        self.b1x = 0.0
        self.b1y = 0.0

        self.b2x = 0.0
        self.b2y = 0.0

        self.b3x = 0.0
        self.b3y = 0.0

        self.b4x = 0.0
        self.b4y = 0.0

        # -----------------------------
        # Subscribers
        # -----------------------------
        self.create_subscription(
            Point,
            '/robot_pose',
            self.robot_callback,
            10
        )

        self.create_subscription(
            Point,
            '/beacon1',
            self.beacon1_callback,
            10
        )

        self.create_subscription(
            Point,
            '/beacon2',
            self.beacon2_callback,
            10
        )

        self.create_subscription(
            Point,
            '/beacon3',
            self.beacon3_callback,
            10
        )

        self.create_subscription(
            Point,
            '/beacon4',
            self.beacon4_callback,
            10
        )

        # -----------------------------
        # Publishers
        # -----------------------------
        self.rssi1_pub = self.create_publisher(Float32, '/beacon1_rssi', 10)
        self.rssi2_pub = self.create_publisher(Float32, '/beacon2_rssi', 10)
        self.rssi3_pub = self.create_publisher(Float32, '/beacon3_rssi', 10)
        self.rssi4_pub = self.create_publisher(Float32, '/beacon4_rssi', 10)

        # -----------------------------
        # Timer
        # -----------------------------
        self.timer = self.create_timer(
            1.0,
            self.calculate_rssi
        )

        self.get_logger().info("RSSI Generator Started")

    # =====================================================
    # CALLBACKS
    # =====================================================

    def robot_callback(self, msg):
        self.robot_x = msg.x
        self.robot_y = msg.y

    def beacon1_callback(self, msg):
        self.b1x = msg.x
        self.b1y = msg.y

    def beacon2_callback(self, msg):
        self.b2x = msg.x
        self.b2y = msg.y

    def beacon3_callback(self, msg):
        self.b3x = msg.x
        self.b3y = msg.y

    def beacon4_callback(self, msg):
        self.b4x = msg.x
        self.b4y = msg.y

    # =====================================================
    # RSSI FUNCTION
    # =====================================================

    def calculate_single_rssi(self, bx, by):

        distance = math.sqrt(
            (self.robot_x - bx) ** 2 +
            (self.robot_y - by) ** 2
        )

        if distance < 0.1:
            distance = 0.1

        RSSI0 = -59
        n = 3

        rssi = RSSI0 - (10 * n * math.log10(distance))

        noise = random.gauss(0, 2)

        rssi += noise

        return rssi, distance

    # =====================================================
    # TIMER
    # =====================================================

    def calculate_rssi(self):

        rssi1, d1 = self.calculate_single_rssi(self.b1x, self.b1y)
        rssi2, d2 = self.calculate_single_rssi(self.b2x, self.b2y)
        rssi3, d3 = self.calculate_single_rssi(self.b3x, self.b3y)
        rssi4, d4 = self.calculate_single_rssi(self.b4x, self.b4y)

        msg1 = Float32()
        msg1.data = float(rssi1)

        msg2 = Float32()
        msg2.data = float(rssi2)

        msg3 = Float32()
        msg3.data = float(rssi3)

        msg4 = Float32()
        msg4.data = float(rssi4)

        self.rssi1_pub.publish(msg1)
        self.rssi2_pub.publish(msg2)
        self.rssi3_pub.publish(msg3)
        self.rssi4_pub.publish(msg4)

        self.get_logger().info(
            f"""
Robot ({self.robot_x:.2f}, {self.robot_y:.2f})

Beacon1 RSSI : {rssi1:.2f} dBm
Beacon2 RSSI : {rssi2:.2f} dBm
Beacon3 RSSI : {rssi3:.2f} dBm
Beacon4 RSSI : {rssi4:.2f} dBm
"""
        )


def main(args=None):

    rclpy.init(args=args)

    node = RSSIGenerator()

    rclpy.spin(node)

    node.destroy_node()

    rclpy.shutdown()


if __name__ == '__main__':
    main()