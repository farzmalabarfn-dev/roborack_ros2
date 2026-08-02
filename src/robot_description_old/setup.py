from setuptools import find_packages, setup
from glob import glob
import os

package_name = 'robot_description'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        (
            'share/ament_index/resource_index/packages',
            ['resource/' + package_name]
        ),

        (
            os.path.join('share', package_name),
            ['package.xml']
        ),

        (
            os.path.join('share', package_name, 'launch'),
            glob('launch/*.py')
        ),

        (
            os.path.join('share', package_name, 'urdf'),
            glob('urdf/*')
        ),

        (
            os.path.join('share', package_name, 'config'),
            glob('config/*')
        ),

        (
            os.path.join('share', package_name, 'rviz'),
            glob('rviz/*')
        ),

        (
            os.path.join('share', package_name, 'worlds'),
            glob('worlds/*')
        ),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='jazib-manzoor',
    maintainer_email='your@email.com',
    description='Robot Description Package',
    license='Apache-2.0',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [],
    },
)