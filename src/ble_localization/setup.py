from setuptools import find_packages, setup

package_name = 'ble_localization'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        (
            'share/ament_index/resource_index/packages',
            ['resource/' + package_name],
        ),
        (
            'share/' + package_name,
            ['package.xml'],
        ),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='jazib-manzoor',
    maintainer_email='your_email@example.com',
    description='BLE Localization Simulator using ROS 2',
    license='Apache-2.0',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'robot_node = ble_localization.robot_node:main',
            'beacon_node = ble_localization.beacon_node:main',
            'rssi_generator = ble_localization.rssi_generator:main',
            'localization_node = ble_localization.localization_node:main',
            'visualizer = ble_localization.visualizer:main',
        ],
    },
)