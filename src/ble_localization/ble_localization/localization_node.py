import math

import rclpy
from rclpy.node import Node

from geometry_msgs.msg import Point
from std_msgs.msg import Float32

from scipy.optimize import least_squares


class LocalizationNode(Node):

    def __init__(self):
        super().__init__('localization_node')

        # ------------------------------------
        # RSSI Values
        # ------------------------------------
        self.rssi1 = -80.0
        self.rssi2 = -80.0
        self.rssi3 = -80.0
        self.rssi4 = -80.0

        # ------------------------------------
        # Beacon Coordinates
        # ------------------------------------
        self.beacons = [
            (0.0, 0.0),
            (10.0, 0.0),
            (0.0, 10.0),
            (10.0, 10.0)
        ]

        # ------------------------------------
        # Subscribers
        # ------------------------------------
        self.create_subscription(
            Float32,
            '/beacon1_rssi',
            self.rssi1_callback,
            10
        )

        self.create_subscription(
            Float32,
            '/beacon2_rssi',
            self.rssi2_callback,
            10
        )

        self.create_subscription(
            Float32,
            '/beacon3_rssi',
            self.rssi3_callback,
            10
        )

        self.create_subscription(
            Float32,
            '/beacon4_rssi',
            self.rssi4_callback,
            10
        )

        # ------------------------------------
        # Publisher
        # ------------------------------------
        self.pose_pub = self.create_publisher(
            Point,
            '/estimated_pose',
            10
        )

        # ------------------------------------
        # Timer
        # ------------------------------------
        self.timer = self.create_timer(
            1.0,
            self.estimate_position
        )

        self.get_logger().info("Localization Node Started")

    # ===================================================
    # CALLBACKS
    # ===================================================

    def rssi1_callback(self, msg):
        self.rssi1 = msg.data

    def rssi2_callback(self, msg):
        self.rssi2 = msg.data

    def rssi3_callback(self, msg):
        self.rssi3 = msg.data

    def rssi4_callback(self, msg):
        self.rssi4 = msg.data

    # ===================================================
    # RSSI -> DISTANCE
    # ===================================================

    def rssi_to_distance(self, rssi):

        RSSI0 = -59
        n = 3

        distance = 10 ** ((RSSI0 - rssi) / (10 * n))

        return distance

    # ===================================================
    # ERROR FUNCTION
    # ===================================================

    def residuals(self, position, beacon_positions, distances):

        x, y = position

        errors = []

        for (bx, by), d in zip(beacon_positions, distances):

            predicted = math.sqrt((x - bx) ** 2 + (y - by) ** 2)

            errors.append(predicted - d)

        return errors

    # ===================================================
    # MAIN LOCALIZATION
    # ===================================================

    def estimate_position(self):

        distances = [

            self.rssi_to_distance(self.rssi1),

            self.rssi_to_distance(self.rssi2),

            self.rssi_to_distance(self.rssi3),

            self.rssi_to_distance(self.rssi4)

        ]

        initial_guess = [5.0, 5.0]

        result = least_squares(

            self.residuals,

            initial_guess,

            args=(self.beacons, distances)

        )

        estimated_x = result.x[0]
        estimated_y = result.x[1]

        msg = Point()

        msg.x = float(estimated_x)
        msg.y = float(estimated_y)
        msg.z = 0.0

        self.pose_pub.publish(msg)

        self.get_logger().info(
            f"Estimated Position: ({estimated_x:.2f}, {estimated_y:.2f})"
        )


def main(args=None):

    rclpy.init(args=args)

    node = LocalizationNode()

    rclpy.spin(node)

    node.destroy_node()

    rclpy.shutdown()


if __name__ == '__main__':
    main()