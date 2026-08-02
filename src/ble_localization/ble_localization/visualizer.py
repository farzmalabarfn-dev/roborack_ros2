import math

import matplotlib.pyplot as plt

import rclpy
from rclpy.node import Node

from geometry_msgs.msg import Point


class Visualizer(Node):

    def __init__(self):

        super().__init__('visualizer')

        # --------------------------
        # Robot Position
        # --------------------------
        self.true_x = 0.0
        self.true_y = 0.0

        # --------------------------
        # Estimated Position
        # --------------------------
        self.est_x = 0.0
        self.est_y = 0.0

        # --------------------------
        # Subscribers
        # --------------------------
        self.create_subscription(
            Point,
            '/robot_pose',
            self.robot_callback,
            10
        )

        self.create_subscription(
            Point,
            '/estimated_pose',
            self.estimated_callback,
            10
        )

        # --------------------------
        # Plot
        # --------------------------
        plt.ion()

        self.fig, self.ax = plt.subplots()

        self.timer = self.create_timer(
            0.2,
            self.update_plot
        )

        self.get_logger().info("Visualizer Started")

    def robot_callback(self, msg):

        self.true_x = msg.x
        self.true_y = msg.y

    def estimated_callback(self, msg):

        self.est_x = msg.x
        self.est_y = msg.y

    def update_plot(self):

        self.ax.clear()

        # Room

        self.ax.set_xlim(0, 10)
        self.ax.set_ylim(0, 10)

        self.ax.set_xlabel("X (m)")
        self.ax.set_ylabel("Y (m)")
        self.ax.set_title("BLE Localization Simulator")

        # Grid

        self.ax.grid(True)

        # --------------------------
        # Beacons
        # --------------------------

        beacons = [

            (0,0),
            (10,0),
            (0,10),
            (10,10)

        ]

        bx = [b[0] for b in beacons]
        by = [b[1] for b in beacons]

        self.ax.scatter(
            bx,
            by,
            s=120,
            marker='^',
            label='BLE Beacon'
        )

        # --------------------------
        # True Robot
        # --------------------------

        self.ax.scatter(

            self.true_x,

            self.true_y,

            s=100,

            label='True Robot'

        )

        # --------------------------
        # Estimated Robot
        # --------------------------

        self.ax.scatter(

            self.est_x,

            self.est_y,

            s=100,

            label='Estimated Robot'

        )

        # --------------------------
        # Error

        error = math.sqrt(

            (self.true_x-self.est_x)**2+

            (self.true_y-self.est_y)**2

        )

        self.ax.text(

            0.3,

            9.4,

            f"Localization Error : {error:.2f} m",

            fontsize=11

        )

        self.ax.legend()

        plt.draw()

        plt.pause(0.01)


def main(args=None):

    rclpy.init(args=args)

    node = Visualizer()

    rclpy.spin(node)

    node.destroy_node()

    rclpy.shutdown()


if __name__ == '__main__':

    main()