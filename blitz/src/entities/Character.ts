import { BaseEntity, Column, Entity, PrimaryColumn } from 'typeorm';

@Entity()
export class Character extends BaseEntity {
  @PrimaryColumn()
  character_id: string;

  @Column()
  img: string;

  @Column({
    nullable: true,
  })
  tier: number;
}
