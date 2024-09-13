import pygame_ecs
import random

class GuessComponent(pygame_ecs.BaseComponent):
    def __init__(self):
        super().__init__()
        self.my_number = random.randint(1, 100)
        self.attempts_left = 5

class PlayerComponent(pygame_ecs.BaseComponent):
    def __init__(self, name):
        super().__init__()
        self.name = name

class GuessSystem(pygame_ecs.BaseSystem):
    def __init__(self):
        super().__init__(required_component_types=[GuessComponent, PlayerComponent])

    def update_entity(self, _ , components):
        guess_component: GuessComponent = components[GuessComponent]
        player_component: PlayerComponent = components[PlayerComponent]

        while guess_component.attempts_left > 0:
            guess = int(input(f"{player_component.name}, bir sayı tahmin et (1-100): "))
            if guess == guess_component.my_number:
                print("Bravoo!!! Doğru tahmin!")
                break
            elif guess < guess_component.my_number:
                print("Daha büyük bir sayı tahmin etmelisin.")
            else:
                print("Daha küçük bir sayı tahmin etmelisin.")
            guess_component.attempts_left -= 1

        if guess_component.attempts_left == 0:
            print(f"Çok üzgünün ama bilemedin :/ Tuttuğum sayı {guess_component.my_number} idi.")

def main():
    component_manager = pygame_ecs.ComponentManager()
    entity_manager = pygame_ecs.EntityManager(component_manager)
    system_manager = pygame_ecs.SystemManager(entity_manager, component_manager)

    player_entity = entity_manager.add_entity()
    component_manager.add_component(player_entity, PlayerComponent("Misafir Oyuncu"))

    game_entity = entity_manager.add_entity()
    component_manager.add_component(game_entity, GuessComponent())

    guess_system = GuessSystem()
    system_manager.add_system(guess_system)

    system_manager.update_entities()

if __name__ == "__main__":
    main()
