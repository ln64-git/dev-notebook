from crewai import Task
from textwrap import dedent

class CustomTasks:
    def __tip_section(self):
        return "If you do your BEST WORK, I'll give you a $10,000 commission!"

    def design_system(self, agent, priority, deadline, app_name, app_description):
        return Task(
            description=dedent(
                f"""
                Design the system architecture for {app_name}.
                
                {self.__tip_section()}

                Ensure the architecture meets all business and technical requirements based on the following description: {app_description}.
            """
            ),
            agent=agent,
            priority=priority,
            deadline=deadline,
        )

    def implement_feature(self, agent, priority, deadline, feature_name):
        return Task(
            description=dedent(
                f"""
                Implement the feature: {feature_name}.
                
                {self.__tip_section()}

                Ensure the feature is implemented correctly and thoroughly tested.
            """
            ),
            agent=agent,
            priority=priority,
            deadline=deadline,
        )

    def test_feature(self, agent, priority, deadline, feature_name):
        return Task(
            description=dedent(
                f"""
                Test the feature: {feature_name}.
                
                {self.__tip_section()}

                Ensure all functionalities of the feature are tested, and report any bugs.
            """
            ),
            agent=agent,
            priority=priority,
            deadline=deadline,
        )

    def deploy_application(self, agent, priority, deadline, app_name):
        return Task(
            description=dedent(
                f"""
                Deploy the application: {app_name}.
                
                {self.__tip_section()}

                Ensure the deployment is smooth, monitor the process, and resolve any issues.
            """
            ),
            agent=agent,
            priority=priority,
            deadline=deadline,
        )

    def schedule_meeting(self, agent, priority, deadline, time):
        return Task(
            description=dedent(
                f"""
                Schedule a meeting at {time}.
                
                {self.__tip_section()}

                Ensure all participants are notified and have the necessary information.
            """
            ),
            agent=agent,
            priority=priority,
            deadline=deadline,
        )

    def compile_report(self, agent, priority, deadline, developers):
        return Task(
            description=dedent(
                f"""
                Compile a progress report for the following developers:
                {', '.join(developers)}.
                
                {self.__tip_section()}
            """
            ),
            agent=agent,
            priority=priority,
            deadline=deadline,
        )
