from setuptools import find_packages, setup
import os
from glob import glob

package_name = 'my_first_robot'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(),
    data_files=[
        (
            'share/ament_index/resource_index/packages',
            ['resource/' + package_name]
        ),
        (
            'share/' + package_name,
            ['package.xml']
        ),
        (
            os.path.join('share', package_name, 'launch'),
            glob('launch/*.launch.py')
        ),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='jazib-manzoor',
    maintainer_email='your_email@example.com',
    description='My first ROS 2 robot package',
    license='TODO',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'hello_node = my_first_robot.helo_node:main',
            'publisher_node = my_first_robot.publisher_node:main',
            'my_first_subscriber = my_first_robot.my_first_subscriber:main',
            'service_server = my_first_robot.service_server:main',
            'service_client = my_first_robot.service_client:main',
            'action_server = my_first_robot.action_server:main',
            'action_client = my_first_robot.action_client:main',
        ],
    },
)